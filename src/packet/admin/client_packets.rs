//! This module contains the definitions for the admin packets that can be
//! sent to the server.

use enum_primitive_derive::Primitive;
use crate::packet::serde::WritablePacket;
use serde_derive::{Serialize, Deserialize};

/// Implemented by all admin client-sendable types.
pub trait Packet: WritablePacket {
    const PACKET_TYPE: u8;
}
impl<T: Packet> WritablePacket for T {
    const PACKET_TYPE: u8 = <T as Packet>::PACKET_TYPE;
}

/// The admin announces and authenticates itself to the server.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Join {
    /// Password the server is expecting for this network.
    pub password: String,
    /// Name of the application being used to connect.
    pub name: String,
    /// Version string of the application being used to connect.
    pub version: String
}
impl Packet for Join {
    const PACKET_TYPE: u8 = 0;
}

/// Notification to the server that this admin is quitting.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Quit;
impl Packet for Quit {
    const PACKET_TYPE: u8 = 1;
}

/// Register updates to be sent at certain frequencies (as announced in the PROTOCOL packet).
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct UpdateFrequency {
    /// Update type (see #AdminUpdateType).
    pub update_type: u16,
    /// Update frequency (see #AdminUpdateFrequency), setting #ADMIN_FREQUENCY_POLL is always ignored.
    pub frequency: u16
}
impl Packet for UpdateFrequency {
    const PACKET_TYPE: u8 = 2;
}

/// Poll the server for certain updates, an invalid poll (e.g. not existent id) gets silently dropped.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct AdminPoll {
    /// #AdminUpdateType the server should answer for, only if #AdminUpdateFrequency #ADMIN_FREQUENCY_POLL is advertised in the PROTOCOL packet.
    pub update_type: u8,
    /// ID relevant to the packet type, e.g.
	/// - the client ID for #ADMIN_UPDATE_CLIENT_INFO. Use UINT32_MAX to show all clients.
	/// - the company ID for #ADMIN_UPDATE_COMPANY_INFO. Use UINT32_MAX to show all companies.
    pub id: u32
}
impl Packet for AdminPoll {
    const PACKET_TYPE: u8 = 3;
}

/// Send chat as the server.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AdminChat {
    /// Action such as NETWORK_ACTION_CHAT_CLIENT (see #NetworkAction).
    pub action: u8,
    /// Destination type such as DESTTYPE_BROADCAST (see #DestType).
    pub destination_type: u8,
    /// ID of the destination such as company or client id.
    pub destination_id: u32,
    /// Message.
    pub message: String
}
impl Packet for AdminChat {
    const PACKET_TYPE: u8 = 4;
}

/// Execute a command on the servers console.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AdminRcon {
    /// Command to be executed.
    pub command: String
}
impl Packet for AdminRcon {
    const PACKET_TYPE: u8 = 5;
}

/// Send a JSON string to the current active GameScript.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Gamescript {
    /// JSON string for the GameScript.
    pub json: String
}
impl Packet for Gamescript {
    const PACKET_TYPE: u8 = 6;
}

/// Ping the server, requiring the server to reply with a pong packet.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ping {
    ///  Integer value to pass to the server, which is quoted in the reply.
    pub id: u32
}
impl Packet for Ping {
    const PACKET_TYPE: u8 = 7;
}