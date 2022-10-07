use binread::BinRead;

use crate::types::shared;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MMID {
    /*
    uint32_t offsets[0];            // filename starting position in MMDX chunk. These entries are getting referenced in the MDDF chunk.
    */
    #[br(parse_with = shared::read_until_end)]
    pub offsets: Vec<u32>,
}