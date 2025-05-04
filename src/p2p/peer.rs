// use core::marker::Copy;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPeer {
    name: String,
    id: Uuid,
}

pub struct Peer {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PeerMessage {
    Connecting,
    Connected,
    Disconnect,
}
const MULTICASE_ADD: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
const PORT: u16 = 54321;

impl LocalPeer {
    pub async fn announce(name: &str) {
        println!("I am announcing myself..."); 
        let peer = LocalPeer {
            name: name.to_string(),
            id: Uuid::new_v4(),
        };

        let socket = UdpSocket::bind("0.0.0.0:55155").await.unwrap();
        let multicase_socket = SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 123), 54321);
        socket.join_multicast_v4(MULTICASE_ADD, Ipv4Addr::UNSPECIFIED).unwrap();
        let mut message = PeerMessage::Connecting;
        let mut buffer = [0u8; 256];
        while let PeerMessage::Connecting = message {
            println!("Trying to connect...");
            socket.send_to(
                serde_json::to_string(&peer).unwrap().as_bytes(),
                multicase_socket,
            ).await.unwrap();
            println!("Sending to multicast socket...");
            match socket.try_recv_from(&mut buffer) {
                Ok((size, result)) => {
                    message =  PeerMessage::Connected;
                },
                Err(_) => {
                    println!("error")
                } 
            }
        }
    }
}
