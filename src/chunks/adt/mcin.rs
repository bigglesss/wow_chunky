use binread::BinRead;

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(little)]
pub struct MCINChunk {
    /*
    uint32_t offset;               // absolute offset.
    uint32_t size;                 // the size of the MCNK chunk, this is refering to.
    uint32_t flags;                // always 0. only set in the client., FLAG_LOADED = 1
    union
    {
        char pad[4];
        uint32_t asyncId;            // not in the adt file. client use only
    };
    */
    offset: u32,
    size: u32,
    flags: u32,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(little)]
pub struct MCIN {
    #[br(count = 16*16)]
    chunks: Vec<MCINChunk>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binread::BinReaderExt;

    const RAW_MCIN: [u8; 12] = [ 0xFA, 0x10, 0x00, 0x00, 0xCC, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];

    #[test]
    fn parse_valid_chunk() {
        let mut cursor = std::io::Cursor::new(RAW_MCIN);
        let chunk = cursor.read_le::<MCINChunk>().unwrap();
        assert_eq!(chunk, MCINChunk {
            offset: 4346,
            size: 1740,
            flags: 0,
        })
    }
}