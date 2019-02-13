//! This module contains the definitions for the admin packets that can be
//! sent by the server. All different types of packets are contained in the
//! enum [`Packet`](crate::packet::admin::server_packets::Packet). Packets that contain extra information also
//! have their own struct.

use crate::types;
use serde_derive::{Deserialize, Serialize};

/// An error was caused by this admin connection (connection gets closed).
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Error {
    /// The error caused.
    pub error_code: u8,
}

/// Describes an update packet the admin client can register for.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct UpdatePacketDescription {
    /// Update packet type.
    pub packet_type: types::AdminUpdateType,
    /// Frequencies allowed for this update packet (bitwise).
    pub frequencies_allowed: types::UpdateFrequencies,
}

/// Inform a just joined admin about the protocol specifics.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Protocol {
    /// Protocol version.
    pub version: u8,
    /// Different update packet descriptions.
    pub update_packets: Vec<UpdatePacketDescription>,
}

/// Welcome a connected admin to the game.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Welcome {
    /// Name of the Server (e.g. as advertised to master server).
    pub server_name: String,
    /// OpenTTD version string.
    pub openttd_version: String,
    /// Server is dedicated.
    pub is_dedicated: bool,
    /// Name of the Map.
    pub map_name: String,
    /// Random seed of the Map.
    pub map_seed: u32,
    /// Landscape of the Map.
    pub map_landscape: u8,
    /// Start date of the Map.
    pub map_start_date: types::Date,
    /// Map width.
    pub map_width: u16,
    /// Map height.
    pub map_height: u16,
}

/// Send the current date of the game.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Date {
    /// Current game date.
    pub date: types::Date,
}

/// Notification of a new client.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ClientJoin {
    /// ID of the new client.
    pub id: u32,
}

/// Client information of a specific client.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ClientInfo {
    /// ID of the client.
    pub id: u32,
    /// Network address of the client.
    pub address: String,
    /// Name of the client.
    pub name: String,
    /// Language of the client.
    pub language: u8,
    /// Date the client joined the game.
    pub date_joined: types::Date,
    /// ID of the company the client is playing as (255 for spectators).
    pub company_id: u8,
}

/// Client update details on a specific client (e.g. after rename or move).
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ClientUpdate {
    /// ID of the client.
    pub id: u32,
    /// Name of the client.
    pub name: String,
    /// ID of the company the client is playing as (255 for spectators).
    pub company_id: u8,
}

/// Notification about a client leaving the game.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct ClientQuit {
    /// ID of the client that just left.
    pub id: u32,
}

/// Notification about a client error (and thus the clients disconnection).
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct ClientError {
    /// ID of the client that made the error.
    pub id: u32,
    /// Error the client made (see NetworkErrorCode).
    pub error: u8,
}

/// Notification of a new company.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct CompanyNew {
    /// ID of the new company.
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CompanyInfo {
    /// ID of the company.
    pub id: u8,
    /// Name of the company.
    pub name: String,
    /// Name of the companies manager.
    pub manager: String,
    /// Main company colour.
    pub color: u8,
    /// Company is password protected.
    pub password_protected: bool,
    /// Year the company was inaugurated.
    pub inaugurated_year: u32,
    /// Company is an AI.
    pub ai: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CompanyUpdate {
    /// ID of the company.
    pub id: u8,
    /// Name of the company.
    pub name: String,
    // Name of the companies manager.
    pub manager: String,
    /// Main company colour.
    pub color: u8,
    /// Company is password protected.
    pub password_protected: bool,
    /// Quarters of bankruptcy.
    pub quarters_bankrupt: u8,
    /// Owner of share 1.
    pub owner_share_1: u8,
    /// Owner of share 2.
    pub owner_share_2: u8,
    /// Owner of share 3.
    pub owner_share_3: u8,
    /// Owner of share 4.
    pub owner_share_4: u8,
}

/// Notification about a removed company (e.g. due to bankruptcy).
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct CompanyRemove {
    /// ID of the company.
    pub id: u8,
    /// Reason for being removed (see #AdminCompanyRemoveReason).
    pub reason: u8,
}

/// Economy update of a specific company.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct CompanyEconomy {
    /// ID of the company.
    pub id: u8,
    /// Money.
    pub money: u64,
    /// Loan.
    pub loan: u64,
    /// Income.
    pub income: i64,
    /// Delivered cargo (this quarter).
    pub delivered_cargo: u16,
    /// Company value (last quarter).
    pub company_value_last: u64,
    /// Performance (last quarter).
    pub performance_last: u16,
    /// Delivered cargo (last quarter).
    pub delivered_cargo_last: u16,
    /// Company value (previous quarter).
    pub company_value_previous: u64,
    /// Performance (previous quarter).
    pub performance_previous: u16,
    /// Delivered cargo (previous quarter).
    pub delivered_previous: u16,
}

/// Company statistics on stations and vehicles.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct CompanyStats {
    /// ID of the company.
    pub id: u8,
    /// Number of trains.
    pub trains: u16,
    /// Number of lorries.
    pub lorries: u16,
    /// Number of busses.
    pub busses: u16,
    /// Number of planes.
    pub planes: u16,
    /// Number of ships.
    pub ships: u16,
    /// Number of train stations.
    pub train_stations: u16,
    /// Number of lorry stations.
    pub lorry_stations: u16,
    /// Number of bus stops.
    pub bus_stops: u16,
    /// Number of airports and heliports.
    pub airports_and_heliports: u16,
    /// Number of harbours.
    pub harbours: u16,
}

/// Send chat from the game into the admin network.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Chat {
    /// Action such as NETWORK_ACTION_CHAT_CLIENT (see #NetworkAction).
    pub action: u8,
    /// Destination type such as DESTTYPE_BROADCAST (see #DestType).
    pub destination: u8,
    /// ID of the client who sent this message.
    pub client: u32,
    /// Message.
    pub message: String,
    /// Money (only when it is a 'give money' action).
    pub money: Option<u64>,
}

/// Result of an rcon command.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Rcon {
    /// Colour as it would be used on the server or a client.
    pub color: u16,
    /// Output of the executed command.
    pub output: String,
}

/// Send what would be printed on the server's console also into the admin network.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Console {
    /// The origin of the text, e.g. "console" for console, or "net" for network related (debug) messages.
    pub origin: String,
    /// Text as found on the console of the server.
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CmdName {
    /// ID of the DoCommand.
    pub id: u16,
    /// Name of DoCommand.
    pub name: String,
}

/// Send DoCommand names to the bot upon request only. Multiple of these
/// packets can follow each other in order to provide all known DoCommand names.
///
/// NOTICE: Data provided with this packet is not stable and will not be
/// treated as such. Do not rely on IDs or names to be constant across
/// different versions / revisions of OpenTTD. Data provided in this packet is
/// for logging purposes only.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CmdNames {
    pub names: Vec<CmdName>,
}

/// Send incoming command packets to the admin network. This is for logging
/// purposes only.
///
/// NOTICE: Data provided with this packet is not stable and will not be
/// across different versions / revisions of OpenTTD.
/// Data provided in this packet is for logging purposes only.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct CmdLogging {
    /// ID of the client sending the command.
    pub client_id: u32,
    /// ID of the company (0..MAX_COMPANIES-1).
    pub company_id: u8,
    /// ID of the command.
    pub command_id: u16,
    /// P1 (variable data passed to the command).
    pub p1: u32,
    /// P2 (variable data passed to the command).
    pub p2: u32,
    /// Tile where this is taking place.
    pub tile: u32,
    /// Text passed to the command.
    pub text: String,
    /// Frame of execution.
    pub execution_frame: u32,
}

/// Send a JSON string to the current active GameScript.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Gamescript {
    /// JSON string for the GameScript.
    pub json: String,
}

/// Notify the admin connection that the rcon command has finished.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RconEnd {
    /// The command as requested by the admin connection.
    pub command: String,
}

/// Send a ping-reply (pong) to the admin that sent us the ping packet.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Pong {
    /// Should be the same as read from the admins ping packet.
    pub id: u32,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub enum Packet {
    /// The server is full (connection gets closed).
    Full,
    /// The source IP address is banned (connection gets closed).
    Banned,
    /// An error was caused by this admin connection (connection gets closed).
    Error(Error),
    /// Inform a just joined admin about the protocol specifics.
    Protocol(Protocol),
    /// Welcome a connected admin to the game.
    Welcome(Welcome),
    /// Notification about a newgame.
    Newgame,
    /// Notification about the server shutting down.
    Shutdown,
    /// Send the current date of the game.
    Date(Date),
    /// Notification of a new client.
    ClientJoin(ClientJoin),
    /// Client information of a specific client.
    ClientInfo(ClientInfo),
    /// Client update details on a specific client (e.g. after rename or move).
    ClientUpdate(ClientUpdate),
    /// Notification about a client leaving the game.
    ClientQuit(ClientQuit),
    /// Notification about a client error (and thus the clients disconnection).
    ClientError(ClientError),
    /// Notification of a new company.
    CompanyNew(CompanyNew),
    /// Company information on a specific company.
    CompanyInfo(CompanyInfo),
    /// Company information of a specific company.
    CompanyUpdate(CompanyUpdate),
    /// Notification about a removed company (e.g. due to bankruptcy).
    CompanyRemove(CompanyRemove),
    /// Economy update of a specific company.
    CompanyEconomy(CompanyEconomy),
    /// Company statistics on stations and vehicles.
    CompanyStats(CompanyStats),
    /// Send chat from the game into the admin network.
    Chat(Chat),
    /// Result of an rcon command.
    Rcon(Rcon),
    /// Send what would be printed on the server's console also into the admin network.
    Console(Console),
    CmdNames(CmdNames),
    CmdLogging(CmdLogging),
    Gamescript(Gamescript),
    /// Notify the admin connection that the rcon command has finished.
    RconEnd(RconEnd),
    /// Send a ping-reply (pong) to the admin that sent us the ping packet.
    Pong(Pong),
    UnknownPacket {
        packet_type: u8,
        buffer: Vec<u8>,
    },
}
