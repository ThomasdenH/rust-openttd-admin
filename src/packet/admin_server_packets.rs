use enum_primitive_derive::Primitive;
use byteorder::{LittleEndian, ReadBytesExt};
use serde_derive::{Serialize, Deserialize};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AdminServerPacket {
    Full,
    Banned,
    Error,
    Protocol,
    Welcome,
    Newgame,
    Shutdown,
    Date,
    ClientJoin,
    ClientInfo,
    ClientUpdate,
    ClientQuit,
    ClientError,
    CompanyNew,
    CompanyInfo(CompanyInfo),
    CompanyUpdate,
    CompanyRemove,
    CompanyEconomy,
    CompanyStats,
    Chat,
    Rcon,
    Console,
    CmdNames, 
    CmdLogging,
    Gamescript,
    RconEnd,
    Pong,
    UnknownPacket { packet_type: u8, buffer: Vec<u8> }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CompanyInfo {
    id: u8,
    name: String,
    manager: String,
    color: u8,
    password_protected: bool,
    inaugurated_year: u32,
    ai: bool
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Error {
    error_code: u8
}
