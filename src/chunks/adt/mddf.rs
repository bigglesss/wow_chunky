use binread::BinRead;

use crate::chunks::shared;


#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(little, repr = u16)]
pub enum MDDFFlags {
    NONE = 0,
    BIODOME = 1,
    SHRUBBERY = 2,
}

#[derive(Clone, Debug, PartialEq, BinRead)]
#[br(little)]
pub struct MDDFPart {
    /*
    uint32_t nameId;              // references an entry in the MMID chunk, specifying the model to use.
                                     if flag mddf_entry_is_filedata_id is set, a file data id instead, ignoring MMID.
    uint32_t uniqueId;            // this ID should be unique for all ADTs currently loaded. Best, they are unique for the whole map. Blizzard has
                                     these unique for the whole game.
   shared::C3Vectorⁱ position;           // This is relative to a corner of the map. Subtract 17066 from the non vertical values and you should start to see
                                     something that makes sense. You'll then likely have to negate one of the non vertical values in whatever
                                     coordinate system you're using to finally move it into place.
   shared::C3Vectorⁱ rotation;           // degrees. This is not the same coordinate system orientation like the ADT itself! (see history.)
    uint16_t scale;               // 1024 is the default size equaling 1.0f.
    uint16_t flags;               // values from enum MDDFFlags.
    */
    pub name_id: u32,
    pub unique_id: u32,
    pub position: shared::C3Vector,
    pub rotation: shared::C3Vector,
    pub scale: u16,
    pub flags: MDDFFlags,
}
#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MDDF {
    #[br(parse_with = shared::read_until_end)]
    pub parts: Vec<MDDFPart>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use binread::BinReaderExt;

    const RAW_MDDF: [u8; 36] = [ 0x67, 0x00, 0x00, 0x00, 0x8D, 0xF4, 0x02, 0x00, 0xB9, 0x40, 0x85, 0x46, 0x46, 0x39, 0x44, 0x42, 0x91, 0x21, 0x80, 0x46, 0x00, 0x00, 0x00, 0x00, 0x1C, 0xFC, 0xAD, 0x42, 0x00, 0x00, 0x00, 0x00, 0x34, 0x04, 0x00, 0x00 ];

    #[test]
    fn parse_valid_chunk() {
        let mut cursor = std::io::Cursor::new(RAW_MDDF);
        let chunk = cursor.read_le::<MDDFPart>().unwrap();
        assert_eq!(chunk, MDDFPart {
            name_id: 103,
            unique_id: 193677,
            position: shared::C3Vector { x: 17056.361, y: 49.05593, z: 16400.783 },
            rotation: shared::C3Vector { x: 0.0, y: 86.9924, z: 0.0 },
            scale: 1076,
            flags: MDDFFlags::NONE,
        })
    }
}