use crate::p2p::peer;

use super::peer::{LocalPeer, Peer};
use std::{collections::HashMap, thread::JoinHandle};
use tokio::net::UdpSocket;
use uuid::Uuid;

pub struct Server {
    connected_peers: HashMap<Uuid, Peer>,
}

impl Server {
    pub async fn new() -> Self {
        Server {
            connected_peers: HashMap::default(),
        }
    }

    async fn listen_to_multicaste() -> () {
        let socket = UdpSocket::bind("0.0.0.0:55156").await.unwrap();
        let mut buffer = [0u8; 1024];
        loop {
            println!("Waiting for the client...");
            match socket.recv_from(&mut buffer).await {
                Ok((size, source)) => {
                    let peer_info = String::from_utf8_lossy(&buffer);
                    println!("{}", peer_info)
                }
                Err(_) => {
                    print!("Something went wrong")
                }             
            }
        }
    }

    pub async fn run(&self) {
        // Start Peer discovery process
        let multicast_handle = tokio::spawn(async move { Self::listen_to_multicaste().await });
        let announce_handle = tokio::spawn(async move { LocalPeer::announce("new").await });
        tokio::try_join!(multicast_handle, announce_handle).unwrap();

        // Start Server

        // Start Client

        // Manage peers process
    }
}
