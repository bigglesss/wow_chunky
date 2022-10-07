use binread::BinRead;

use crate::chunks::shared;


#[derive(Clone, Debug, BinRead)]
#[br(little, repr = u16)]
pub enum MDDFFlags {
    NONE = 0,
    BIODOME = 1,
    SHRUBBERY = 2,
}

#[derive(Clone, Debug, BinRead)]
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