use crate::packet::write::PacketWriter;
use crate::consts::PacketAdminClientType;

pub struct AdminJoin {
    password: String,
    user: String,
    version: String
}

impl AdminJoin {
    pub fn new(password: String, user: String, version: String) -> AdminJoin {
        AdminJoin {
            password, user, version
        }
    }
}

impl From<AdminJoin> for Vec<u8> {
    fn from(packet: AdminJoin) -> Self {
        let mut writer = PacketWriter::new(PacketAdminClientType::AdminJoin);
        writer.write_string(&packet.password);
        writer.write_string(&packet.user);
        writer.write_string(&packet.version);
        writer.build()
    }
}
