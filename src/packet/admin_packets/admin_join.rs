use std::io::Result;
use crate::packet::write::PacketWriter;

pub struct AdminJoin {
    password: String,
    user: String,
    version: String
}

impl AdminJoin {
    fn new(password: String, user: String, version: String) -> AdminJoin {
        AdminJoin {
            password, user, version
        }
    }

    fn to_buffer(self) -> Result<Vec<u8>> {
        let mut writer = PacketWriter::new();
        writer.write_string(self.password)?;
        writer.write_string(self.user)?;
        writer.write_string(self.version)?;
        Ok(writer.build())
    }
}
