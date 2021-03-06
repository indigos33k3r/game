// Standard
use std::{thread, time::Duration};

// Library
use parking_lot::Mutex;
use vek::*;

// Project
use common::{
    terrain::Entity,
    util::{
        manager::Manager,
        msg::{ClientMsg, CompStore, ServerMsg, SessionKind},
        post::Incoming,
    },
};

// Local
use crate::{Client, ClientEvent, ClientStatus, Payloads};

// Constants
const PING_TIMEOUT: Duration = Duration::from_secs(10);
const PING_FREQ: Duration = Duration::from_secs(2);

impl<P: Payloads> Client<P> {
    pub(crate) fn handle_incoming(&self, mgr: &mut Manager<Self>) {
        while let Ok(incoming) = self.postoffice.await_incoming() {
            match incoming {
                // Sessions
                Incoming::Session(session) => match session.kind {
                    SessionKind::Ping => {
                        let pb = Mutex::new(session.postbox);
                        // TODO: Move this to a dedicated method?
                        Manager::add_worker(mgr, |_client, _running, _| {
                            thread::spawn(move || {
                                let pb = pb.into_inner();

                                loop {
                                    thread::sleep(PING_FREQ);
                                    let _ = pb.send(ClientMsg::Ping);

                                    match pb.recv_timeout(PING_TIMEOUT) {
                                        Ok(ServerMsg::Ping) => {},
                                        _ => break, // Anything other than a ping over this session is invalid
                                    }
                                }
                            });
                        })
                    },
                    _ => {},
                },

                // One-shot messages
                Incoming::Msg(ServerMsg::ChatMsg { text }) => {
                    self.events.lock().push(ClientEvent::RecvChatMsg { text })
                },
                Incoming::Msg(ServerMsg::CompUpdate { uid, store }) => {
                    let entity = self.entity(uid).unwrap_or_else(|| {
                        // Create an entity with default attributes if it doesn't already exist
                        self.add_entity(
                            uid,
                            Entity::new(Vec3::zero(), Vec3::zero(), Vec3::zero(), Vec2::unit_y()),
                        );
                        // This shouldn't be able to fail since we just created the entity. If it
                        // does (because this is *technically* a data race)... then damn. Unlucky.
                        self.entity(uid).unwrap()
                    });

                    match store {
                        CompStore::Pos(pos) => *entity.write().pos_mut() = pos,
                        CompStore::Vel(vel) => *entity.write().vel_mut() = vel,
                        CompStore::Dir(dir) => *entity.write().look_dir_mut() = dir,
                        _ => {},
                    }
                },
                Incoming::Msg(ServerMsg::EntityDeleted { uid }) => {
                    self.remove_entity(uid);
                },

                Incoming::Msg(ServerMsg::TimeUpdate(time)) => {
                    *self.clock_tick_time.write() = time;
                    self.clock.write().reset();
                },

                Incoming::Msg(_) => {},

                // End
                Incoming::End => {}, // TODO: Something here
            }
        }

        *self.status.write() = ClientStatus::Disconnected;
    }

    /// Update the server with information about the player
    pub(crate) fn update_server(&self) {
        if let Some(player_entity) = self.player_entity() {
            let player_entity = player_entity.read();
            let _ = self.postoffice.send_one(ClientMsg::PlayerEntityUpdate {
                pos: *player_entity.pos(),
                vel: *player_entity.vel(),
                dir: *player_entity.look_dir(),
            });
        }
    }
}
