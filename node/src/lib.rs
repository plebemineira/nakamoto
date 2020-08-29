pub mod error;
pub mod handle;
pub mod node;

use std::net;
use std::time;

pub use nakamoto_p2p::address_book::AddressBook;
pub use nakamoto_p2p::protocol::bitcoin::Network;

#[cfg(test)]
mod tests;

pub fn run(
    connect: &[net::SocketAddr],
    listen: &[net::SocketAddr],
    network: Network,
) -> Result<(), error::Error> {
    use node::*;

    let address_book = if connect.is_empty() {
        match AddressBook::load("peers") {
            Ok(peers) if peers.is_empty() => {
                log::info!("Address book is empty. Trying DNS seeds..");
                AddressBook::bootstrap(network.seeds(), network.port())?
            }
            Ok(peers) => peers,
            Err(err) => {
                return Err(error::Error::AddressBook(err));
            }
        }
    } else {
        AddressBook::from(connect)?
    };

    let node = Node::new(NodeConfig {
        discovery: true,
        network,
        listen: if listen.is_empty() {
            vec![([0, 0, 0, 0], 0).into()]
        } else {
            listen.to_vec()
        },
        address_book,
        timeout: time::Duration::from_secs(30),
        ..NodeConfig::default()
    })?;

    node.run()
}
