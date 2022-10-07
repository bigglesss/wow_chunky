use binread::BinRead;

use crate::chunks::shared;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MTEX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to textures. Referenced in MCLY.
    */
    #[br(parse_with = shared::zero_terminated_strings)]
    pub filenames: Vec<String>,
}
