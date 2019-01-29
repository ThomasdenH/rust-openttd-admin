pub mod packet;
pub mod consts;

use crate::packet::admin_packets::AdminJoin;
use crate::packet::read::ReadAdminServerPacket;
use std::net::TcpStream;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut stream = TcpStream::connect("localhost:")?;

    let join = AdminJoin::new(
        "banaan".to_string(),
        "rust".to_string(),
        "1.8.0".to_string()
    );
    stream.write(&join.to_buffer())?;
    let packet = stream.read_packet()?;

    Ok(())
}
