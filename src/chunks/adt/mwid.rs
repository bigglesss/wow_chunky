use binread::BinRead;

use crate::chunks::shared;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MWID {
    /*
    uint32_t offsets[0];            // filename starting position in MWMO chunk. These entries are getting referenced in the MODF chunk.
    */
    #[br(parse_with = shared::read_until_end)]
    pub offsets: Vec<u32>,
}