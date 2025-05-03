use crate::p2p::peer;

use super::peer::{Peer, LocalPeer};
use std::{collections::HashMap, net::UdpSocket, thread::JoinHandle};
use uuid::Uuid;

pub struct Server {
    connected_peers: HashMap<Uuid, Peer>,
}

impl Server {
    pub async fn new() -> Self {
        Server {
            connected_peers: HashMap::default()
        }
    }

    async fn listen_to_multicase() -> (){
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let mut buffer = [0u8; 1024];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    let peer_info = String::from_utf8_lossy(&buffer);
                    println!("{}", peer_info)
                },
                Err(_) => {
                    print!("Something went wrong")
                }
            }
        }   
    }

    pub async fn run(&self) {


        // Start Peer discovery process
        tokio::spawn(Self::listen_to_multicase()).await;
        tokio::spawn(LocalPeer::announce("new")).await;
        // Start Server

        // Start Client

        // Manage peers process
    }
}
