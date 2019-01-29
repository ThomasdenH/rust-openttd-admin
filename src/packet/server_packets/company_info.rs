use crate::packet::read::OpenTTDRead;
use byteorder::{ReadBytesExt};
use failure::Error;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CompanyInfo {
    company_index: u8,
    company_name: String,
    manager_name: String,
    colour: u8,
    is_ai: bool,
    quarters_of_bankruptcy: u8,
    shareowners: Vec<u8>
}

impl CompanyInfo {
    pub fn from_buffer(mut buffer: &[u8]) -> Result<CompanyInfo, Error> {
        Ok(CompanyInfo {
            company_index: buffer.read_u8()?,
            company_name: buffer.read_string()?,
            manager_name: buffer.read_string()?,
            colour: buffer.read_u8()?,
            is_ai: buffer.read_bool()?,
            quarters_of_bankruptcy: buffer.read_u8()?,
            shareowners: buffer.to_vec()
        })
    }
}
