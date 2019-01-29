
/// Mirrors the enum [PacketAdminType](https://github.com/OpenTTD/OpenTTD/blob/master/src/network/core/tcp_admin.h).
#[repr(u8)]
pub enum PacketAdminType {
    /// The admin announces and authenticates itself to the server.
	AdminJoin,
    /// The admin tells the server that it is quitting.         
	AdminQuit,
    /// The admin tells the server the update frequency of a particular piece of information.             
	AdminUpdateFrequency, 
    /// The admin explicitly polls for a piece of information.
	AdminPoll,             
    /// The admin sends a chat message to be distributed.
	AdminChat,     
    /// The admin sends a remote console command.        
	AdminRcon,             
    /// The admin sends a JSON string for the GameScript.
	AdminGamescript,       
    /// The admin sends a ping to the server, expecting a ping-reply (PONG) packet.
	AdminPing,

    /// The server tells the admin it cannot accept the admin.
	ServerFull = 100,      
    /// The server tells the admin it is banned.
	ServerBanned,          
    /// The server tells the admin an error has occurred.
	ServerError,           
    /// The server tells the admin its protocol version.
	ServerProtocol,    
    /// The server welcomes the admin to a game.    
	ServerWelcome,         
    /// The server tells the admin its going to start a new game.
	ServerNewgame,         
    /// The server tells the admin its shutting down.
	ServerShutdown,        

    /// The server tells the admin what the current game date is.
	ServerDate,            
    /// The server tells the admin that a client has joined.
	ServerClientJoin,    
    /// The server gives the admin information about a client. 
	ServerClientInfo,     
    /// The server gives the admin an information update on a client.
	ServerClientUpdate,   
    /// The server tells the admin that a client quit.
	ServerClientQuit,     
    /// The server tells the admin that a client caused an error.
	ServerClientError,    
    /// The server tells the admin that a new company has started.
	ServerCompanyNew,     
    /// The server gives the admin information about a company.
	ServerCompanyInfo,    
    /// The server gives the admin an information update on a company.
	ServerCompanyUpdate,  
    /// The server tells the admin that a company was removed.
	SeverCompanyRemove,  
    /// The server gives the admin some economy related company information.
	ServerCompanyEconomy, 
    /// The server gives the admin some statistics about a company.
	ServerCompanyStats,   
    /// The server received a chat message and relays it.
	ServerChat,            
    /// The server's reply to a remove console command.
	ServerRcon,            
    /// The server gives the admin the data that got printed to its console.
	ServerConsole,         
    /// The server sends out the names of the DoCommands to the admins.
	ServerCmdNames,    
    /// The server gives the admin copies of incoming command packets.   
	ServerCmdLogging,     
    /// The server gives the admin information from the GameScript in JSON.
	ServerGamescript,      
    /// The server indicates that the remote console command has completed.
	ServerRconEnd,        
    /// The server replies to a ping request from the admin.
	ServerPong      
}
