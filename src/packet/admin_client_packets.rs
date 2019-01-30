use enum_primitive_derive::Primitive;

/// Admin packets sent by the client to the admin server.
#[derive(Primitive, Copy, Clone, Eq, PartialEq, Debug)]
pub enum PacketAdminClientType {
    /// The admin announces and authenticates itself to the server.
	AdminJoin = 0,
    /// The admin tells the server that it is quitting.         
	AdminQuit = 1,
    /// The admin tells the server the update frequency of a particular piece of information.             
	AdminUpdateFrequency = 2, 
    /// The admin explicitly polls for a piece of information.
	AdminPoll = 3,             
    /// The admin sends a chat message to be distributed.
	AdminChat = 4,     
    /// The admin sends a remote console command.        
	AdminRcon = 5,             
    /// The admin sends a JSON string for the GameScript.
	AdminGamescript = 6,       
    /// The admin sends a ping to the server, expecting a ping-reply (PONG) packet.
	AdminPing = 7,
}
