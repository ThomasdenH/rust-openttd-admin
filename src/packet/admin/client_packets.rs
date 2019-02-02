//! This module contains the definitions for the admin packets that can be
//! sent to the server.

use crate::packet::serde::WritablePacket;
use crate::types;
use serde_derive::{Deserialize, Serialize};

/// Implemented by all admin client-sendable types.
pub trait Packet: WritablePacket {
    const PACKET_TYPE: u8;
}
impl<T: Packet> WritablePacket for T {
    const PACKET_TYPE: u8 = <T as Packet>::PACKET_TYPE;
}

/// The admin announces and authenticates itself to the server.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Join<'a> {
    /// Password the server is expecting for this network.
    pub password: &'a str,
    /// Name of the application being used to connect.
    pub name: &'a str,
    /// Version string of the application being used to connect.
    pub version: &'a str,
}
impl<'a> Packet for Join<'a> {
    const PACKET_TYPE: u8 = 0;
}

/// Notification to the server that this admin is quitting.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Quit;
impl Packet for Quit {
    const PACKET_TYPE: u8 = 1;
}

/// Register updates to be sent at certain frequencies (as announced in the PROTOCOL packet).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct UpdateFrequency {
    /// Update type (see [`AdminUpdateType`]).
    pub update_type: types::AdminUpdateType,
    /// Update frequency (see #AdminUpdateFrequency), setting #ADMIN_FREQUENCY_POLL is always ignored.
    pub frequency: types::UpdateFrequencies,
}
impl Packet for UpdateFrequency {
    const PACKET_TYPE: u8 = 2;
}

/// Poll the server for certain updates, an invalid poll (e.g. not existent id) gets silently dropped.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Poll {
    /// [`AdminUpdateType`] the server should answer for, only if #AdminUpdateFrequency #ADMIN_FREQUENCY_POLL is advertised in the PROTOCOL packet.
    pub update_type: types::AdminUpdateType,
    /// ID relevant to the packet type, e.g.
    /// - the client ID for #ADMIN_UPDATE_CLIENT_INFO. Use UINT32_MAX to show all clients.
    /// - the company ID for #ADMIN_UPDATE_COMPANY_INFO. Use UINT32_MAX to show all companies.
    pub id: u32,
}
impl Packet for Poll {
    const PACKET_TYPE: u8 = 3;
}

/// Send chat as the server.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Chat<'a> {
    /// Action such as NETWORK_ACTION_CHAT_CLIENT (see #NetworkAction).
    pub action: u8,
    /// Destination type such as DESTTYPE_BROADCAST (see #DestType).
    pub destination_type: u8,
    /// ID of the destination such as company or client id.
    pub destination_id: u32,
    /// Message.
    pub message: &'a str,
}
impl Packet for Chat<'_> {
    const PACKET_TYPE: u8 = 4;
}

/// Execute a command on the servers console.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rcon<'a> {
    /// Command to be executed.
    pub command: &'a str,
}
impl Packet for Rcon<'_> {
    const PACKET_TYPE: u8 = 5;
}

/// Send a JSON string to the current active GameScript.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Gamescript<'a> {
    /// JSON string for the GameScript.
    pub json: &'a str,
}
impl Packet for Gamescript<'_> {
    const PACKET_TYPE: u8 = 6;
}

/// Ping the server, requiring the server to reply with a pong packet.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ping {
    ///  Integer value to pass to the server, which is quoted in the reply.
    pub id: u32,
}
impl Packet for Ping {
    const PACKET_TYPE: u8 = 7;
}
