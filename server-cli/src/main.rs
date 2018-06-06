#![feature(nll)]

#[macro_use] extern crate log;
extern crate pretty_env_logger;
extern crate server;
extern crate get_if_addrs;

use std::io;
use std::net::SocketAddr;
use server::ServerHandle;

const PORT: u16 = 59003;

fn main() {
    pretty_env_logger::init();
    info!("Started server-cli...");

    let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0));
    println!("Hosting on {}:{}...", ip.to_string(), PORT);

    let mut server = ServerHandle::new(SocketAddr::new(ip, PORT), 1227, 1024)
        .expect("Could not create server");

    server.run();
}
