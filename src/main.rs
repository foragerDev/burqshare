use tokio;

pub mod p2p;

use p2p::network::Server;
#[tokio::main]
async fn main() {
    let server = Server::new().await;
    server.run().await;
}
