pub mod packet;
pub mod types;

use crate::packet::admin::{client_packets, server_packets, AdminRead, AdminWrite};
use failure::Error;
use std::net::TcpStream;

fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect("localhost:3977")?;
    stream.write_packet(&client_packets::Join {
        password: "password",
        version: "1.8.0",
        name: "rust",
    })?;
    stream.write_packet(&client_packets::UpdateFrequency {
        frequency: types::UpdateFrequencies::Daily,
        update_type: types::AdminUpdateType::Date,
    })?;
    loop {
        let packet = stream.read_packet()?;
        use server_packets::Packet::*;
        match packet {
            Date(date) => {
                println!("Date: {}", date.date);
            }
            _ => println!("{:?}", packet),
        }
    }
}
