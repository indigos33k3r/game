// Standard
use std::{
    collections::HashMap,
    net::{TcpStream, ToSocketAddrs, Shutdown},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, RecvError, RecvTimeoutError, SendError},
        Arc, Mutex,
    },
    thread,
    fmt::Debug,
    io,
    io::{Write, Read},
    time::Duration,
};

// Local
use net::{Connection, Error, Message, UdpMgr};
use manager::{Managed, Manager};

// Information
// -----------
//
// The post system is a one-to-many relationship between a single postoffice and many postboxes.
// We use mpscs to communicate between each: the outgoing mpsc is owned by the postoffice, the
// incoming mpscs are owned by each postbox respectively. When a message comes in, it gets sent
// off to the corresponding postbox's receiving mpsc. When a message gets sent from a postbox, it
// gets sent through the postoffice's outgoing mpsc.
//
//      ,--> PostBox
//     v
// PostOffice <---> PostBox
//     ^
//     `--> PostBox

// Letter

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Letter<SK, M> {
    OpenBox { uid: u64, kind: SK },
    CloseBox(u64),
    Message { uid: u64, payload: M },
    OneShot(M),
}

impl<SK: Message, M: Message> Message for Letter<SK, M> {}

// PostBoxSession

pub struct PostBoxSession<SK: Message, SM: Message, RM: Message> {
    pub postbox: PostBox<SK, SM, RM>,
    pub kind: SK,
}

// PostBox

pub struct PostBox<SK: Message, SM: Message, RM: Message> {
    uid: u64,
    // The recv end for the incoming mpsc
    recv: mpsc::Receiver<RM>,
    // The send end for the PostOffice outgoing mpsc
    po_send: mpsc::Sender<Result<Letter<SK, SM>, ()>>,
}

impl<SK: Message, SM: Message, RM: Message> PostBox<SK, SM, RM> {
    pub fn send(&self, msg: SM) -> Result<(), SendError<Result<Letter<SK, SM>, ()>>> {
        self.po_send.send(Ok(Letter::Message {
            uid: self.uid,
            payload: msg,
        }))
    }

    pub fn recv(&self) -> Result<RM, RecvError> { self.recv.recv() }

    pub fn recv_timeout(&self, duration: Duration) -> Result<RM, RecvTimeoutError> {
        self.recv.recv_timeout(duration)
    }

    pub fn close(self) -> Result<(), SendError<Result<Letter<SK, SM>, ()>>> {
        self.po_send.send(Ok(Letter::CloseBox(self.uid)))
    }
}

impl<SK: Message, SM: Message, RM: Message> Drop for PostBox<SK, SM, RM> {
    fn drop(&mut self) { let _ = self.po_send.send(Ok(Letter::CloseBox(self.uid))); }
}

// PostOffice

pub struct PostOffice<SK: Message, SM: Message, RM: Message> {
    uid_counter: AtomicU64,

    // The send + recv ends of the outgoing mpsc, used for cloning and passing to postboxes
    outgoing_send: Mutex<mpsc::Sender<Result<Letter<SK, SM>, ()>>>,
    outgoing_recv: Mutex<mpsc::Receiver<Result<Letter<SK, SM>, ()>>>,

    // The send + recv ends of the incoming mpsc, used for cloning and passing to postboxes
    incoming_send: Mutex<mpsc::Sender<Incoming<SK, SM, RM>>>,
    incoming_recv: Mutex<mpsc::Receiver<Incoming<SK, SM, RM>>>,

    // The send ends for the PostBox incoming mpscs
    pb_sends: Mutex<HashMap<u64, mpsc::Sender<RM>>>,

    // Internal connection used for networking
    conn: Arc<Connection<Letter<SK, RM>>>,
}

pub enum Incoming<SK: Message, SM: Message, RM: Message> {
    Session(PostBoxSession<SK, SM, RM>),
    Msg(RM),
    End,
}

impl<SK: Message, SM: Message, RM: Message> PostOffice<SK, SM, RM> {
    // Create a postoffice that runs on the client, talking to a server
    pub fn to_server<U: ToSocketAddrs>(remote_addr: U) -> Result<Manager<PostOffice<SK, SM, RM>>, Error> {
        // Client-side UIDs start from 1 and count odds
        Ok(Manager::init(PostOffice::new_internal(
            1,
            //TcpStream::connect(remote_addr)?,
            Connection::new(&remote_addr, UdpMgr::new())?,
        )?))
    }

    // Create a postoffice that runs on the server, talking to a client
    pub fn to_client(stream: TcpStream) -> Result<Manager<PostOffice<SK, SM, RM>>, Error> {
        // Server-side UIDs start from 0 and count evens
        Ok(Manager::init(PostOffice::new_internal(
            0,
            //stream,
            Connection::new_stream(stream, UdpMgr::new())?,
        )?))
    }

    // Create a postoffice with a few characteristics
    pub fn new_internal(start_uid: u64, conn: Arc<Connection<Letter<SK, RM>>>) -> Result<PostOffice<SK, SM, RM>, io::Error> {
        // Start the internal connection
        Connection::start(&conn);

        // Create the mpsc for outgoing messages
        let (outgoing_send, outgoing_recv) = mpsc::channel();
        let (outgoing_send, outgoing_recv) = (Mutex::new(outgoing_send), Mutex::new(outgoing_recv));

        // Create the mpsc for outgoing messages
        let (incoming_send, incoming_recv) = mpsc::channel();
        let (incoming_send, incoming_recv) = (Mutex::new(incoming_send), Mutex::new(incoming_recv));

        // Create shared running flag
        let running = Arc::new(AtomicBool::new(true));

        Ok(PostOffice {
            uid_counter: AtomicU64::new(start_uid),
            outgoing_send,
            outgoing_recv,
            incoming_send,
            incoming_recv,
            pb_sends: Mutex::new(HashMap::new()),
            conn,
        })
    }

    // Utility to generate a new postbox UID. Server uses even integers, client uses odd integers.
    fn gen_uid(&self) -> u64 { self.uid_counter.fetch_add(2, Ordering::Relaxed) }

    // Utility to create a new postbox with a predetermined UID (not visible to the user)
    fn create_postbox_with_uid(&self, uid: u64) -> PostBox<SK, SM, RM> {
        let (pb_send, pb_recv) = mpsc::channel();
        self.pb_sends.lock().unwrap().insert(uid, pb_send);

        PostBox {
            uid,
            recv: pb_recv,
            po_send: self.outgoing_send.lock().unwrap().clone(),
        }
    }

    // Create a new master postbox, triggering the creation of a slave postbox on the other end
    pub fn create_postbox(&self, kind: SK) -> PostBox<SK, SM, RM> {
        let uid = self.gen_uid();
        let _ = self.outgoing_send.lock().unwrap().send(Ok(Letter::OpenBox::<SK, SM> { uid, kind }));
        self.create_postbox_with_uid(uid)
    }

    // Handle incoming packets, returning any new incoming postboxes as they get created
    pub fn await_incoming(&self) -> Result<Incoming<SK, SM, RM>, RecvError> {
        // Keep receiving messages, relaying the messages to their corresponding target postbox.
        // If Letter::OpenBox or Letter::Message are received, handle them appropriately
        self.incoming_recv.lock().unwrap().recv()
    }

    // Send a single one-off message to the remote postoffice
    pub fn send_one(&self, msg: SM) -> Result<(), SendError<Result<Letter<SK, SM>, ()>>> {
        self.outgoing_send.lock().unwrap().send(Ok(Letter::OneShot(msg)))
    }
}

impl<SK: Message, SM: Message, RM: Message> Managed for PostOffice<SK, SM, RM> {
    fn init_workers(&self, mgr: &mut Manager<Self>) {
        // Create a worker to relay outgoing messages to the connection
        Manager::add_worker(mgr, |po, running, _| {
            // Hold the outgoing receiver permanently
            let outgoing_recv = po.outgoing_recv.lock().unwrap();
            while running.load(Ordering::Relaxed) {
                match outgoing_recv.recv() {
                    Ok(Ok(letter)) => po.conn.send(letter),
                    Ok(Err(_)) | Err(_) => break,
                };
            }
        });

        // Create a worker to relay incoming messages from the connection
        Manager::add_worker(mgr, |po, running, _| {
            // Hold the incoming sender permanently
            let incoming_send = po.incoming_send.lock().unwrap();
            while running.load(Ordering::Relaxed) {
                match po.conn.recv() {
                    Ok(Letter::OpenBox { uid, kind }) => {
                        incoming_send.send(Incoming::Session(PostBoxSession {
                            postbox: po.create_postbox_with_uid(uid),
                            kind,
                        }));
                    },
                    Ok(Letter::CloseBox(uid)) => {
                        po.pb_sends.lock().unwrap().remove(&uid);
                    },
                    Ok(Letter::Message { uid, payload }) => {
                        po.pb_sends.lock().unwrap().get(&uid).map(|s| s.send(payload));
                    },
                    Ok(Letter::OneShot(m)) => {
                        incoming_send.send(Incoming::Msg(m));
                    },
                    Err(_) => break,
                }
            }

            // Send an end message to notify the user that the other end has disconnected
            incoming_send.send(Incoming::End);
        });
    }

    fn on_drop(&self, mgr: &mut Manager<Self>) {
        Manager::internal(mgr).outgoing_send.lock().unwrap().send(Err(()));
        Connection::stop(&Manager::internal(mgr).conn);
    }
}
