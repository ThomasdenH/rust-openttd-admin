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

/// Admin packets sent by the server to the client.
#[derive(Primitive, Copy, Clone, Eq, PartialEq, Debug)]
pub enum PacketAdminServerType {
    /// The server tells the admin it cannot accept the admin.
	ServerFull = 100,      
    /// The server tells the admin it is banned.
	ServerBanned = 101,          
    /// The server tells the admin an error has occurred.
	ServerError = 102,           
    /// The server tells the admin its protocol version.
	ServerProtocol = 103,    
    /// The server welcomes the admin to a game.    
	ServerWelcome = 104,         
    /// The server tells the admin its going to start a new game.
	ServerNewgame = 105,         
    /// The server tells the admin its shutting down.
	ServerShutdown = 106,        

    /// The server tells the admin what the current game date is.
	ServerDate = 107,            
    /// The server tells the admin that a client has joined.
	ServerClientJoin = 108,    
    /// The server gives the admin information about a client. 
	ServerClientInfo = 109,     
    /// The server gives the admin an information update on a client.
	ServerClientUpdate = 110,   
    /// The server tells the admin that a client quit.
	ServerClientQuit = 111,     
    /// The server tells the admin that a client caused an error.
	ServerClientError = 112,    
    /// The server tells the admin that a new company has started.
	ServerCompanyNew = 113,     
    /// The server gives the admin information about a company.
	ServerCompanyInfo = 114,    
    /// The server gives the admin an information update on a company.
	ServerCompanyUpdate = 115,  
    /// The server tells the admin that a company was removed.
	SeverCompanyRemove = 116,  
    /// The server gives the admin some economy related company information.
	ServerCompanyEconomy = 117, 
    /// The server gives the admin some statistics about a company.
	ServerCompanyStats = 118,   
    /// The server received a chat message and relays it.
	ServerChat = 119,            
    /// The server's reply to a remove console command.
	ServerRcon = 120,            
    /// The server gives the admin the data that got printed to its console.
	ServerConsole = 121,         
    /// The server sends out the names of the DoCommands to the admins.
	ServerCmdNames = 122,    
    /// The server gives the admin copies of incoming command packets.   
	ServerCmdLogging = 123,     
    /// The server gives the admin information from the GameScript in JSON.
	ServerGamescript = 124,      
    /// The server indicates that the remote console command has completed.
	ServerRconEnd = 125,        
    /// The server replies to a ping request from the admin.
	ServerPong = 126      
}
