pub mod packet;
pub mod network;

use network::{Configuration, Socket};

fn main() -> Result<()> {
    let socket = Socket::new();

    Ok(())
}
