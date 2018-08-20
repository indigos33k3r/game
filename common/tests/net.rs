// Crates
extern crate common;
#[macro_use]
extern crate serde_derive;

// Standard
use std::{net::TcpListener, sync::Arc, thread, time::Duration};

// Project
use common::{
    net::Message,
    post::{PostBox, PostOffice},
    session::SessionKind,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum ClientMsg {
    Ping,
}
impl Message for ClientMsg {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
enum ServerMsg {
    Pong,
}
impl Message for ServerMsg {}

#[test]
fn post_office() {
    // SERVER
    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || handle_client(PostOffice::new_host(stream).unwrap()));
                },
                Err(e) => panic!("Connection error: {}", e),
            }
        }
    });

    // REMOTE
    handle_remote(PostOffice::new_remote("127.0.0.1:8888").unwrap());
}

fn handle_client(postoffice: Arc<PostOffice<ServerMsg, ClientMsg>>) {
    PostOffice::start(postoffice.clone());

    while let Ok(session) = postoffice.await_incoming() {
        let pb = session.postbox;
        match session.kind {
            SessionKind::PingPong => thread::spawn(move || handle_pingpong(pb)),
        };
    }

    PostOffice::stop(postoffice.clone());
}

fn handle_pingpong(pb: PostBox<ServerMsg, ClientMsg>) {
    while let Ok(msg) = pb.recv() {
        assert_eq!(msg, ClientMsg::Ping);
        let _ = pb.send(ServerMsg::Pong);
    }
}

fn handle_remote(postoffice: Arc<PostOffice<ClientMsg, ServerMsg>>) {
    PostOffice::start(postoffice.clone());

    let po = postoffice.clone();
    thread::spawn(move || {
        while let Ok(_pb) = po.await_incoming() {
            // Handle server sessions
        }
    });

    thread::sleep(Duration::from_millis(1000)); // Waiting for connection

    for _ in 0..10 {
        let pb_r = postoffice.create_postbox(SessionKind::PingPong);

        let _ = pb_r.send(ClientMsg::Ping);
        let msg = pb_r.recv().unwrap();
        assert_eq!(ServerMsg::Pong, msg);

        let _ = pb_r.send(ClientMsg::Ping);
        let msg = pb_r.recv().unwrap();
        assert_eq!(ServerMsg::Pong, msg);
    }
}
