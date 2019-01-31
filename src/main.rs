pub mod packet;

use crate::packet::admin::*;
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("localhost:3977")?;

    let join = client_packets::Join {
        password: "password".to_string(),
        name: "rust-openttd-admin".to_string(),
        version: "1.8.0".to_string(),
    };
    stream.write_packet(&join)?;
    loop {
        let packet = stream.read_packet()?;
        println!("{:?}", packet);
    }
}
