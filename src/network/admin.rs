use serde_derive::{Serialize, Deserialize};
use crate::packet::admin::*;

pub trait PacketListener {
    fn on_packet(packet: client_packets::Packet);
}

#[derive(Debug, Clone)]
pub struct Socket<Listener> where T: PacketListener {
    stream: TcpStream,
    listener: Listener,
    listen_thread: JoinHandle<()>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    address: &str,
    port: u16,
    password: &str,
    name: &str,
    version: &str
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            address: "localhost",
            port: 3977,
            password: "password",
            name: "rust-openttd-admin",
            version: "1.8.0",
        }
    }
}

impl Socket {
    fn connect(config: &Configuration, listener: T) -> Socket<T> where T: PacketListener {
        let mut stream = TcpStream::connect((config.address, config.port))?;

        let join = client_packets::Join {
            password: config.password,
            name: config.name,
            version: config.version,
        };
        stream.write_packet(&join)?;

        let join_handle = thread::spawn(move || {
            loop {
                let packet = stream.read_packet()?;
            }
        });

        Socket {
            stream,
            listener,
            join_handle
        }
    }
}
