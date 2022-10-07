use binread::BinRead;

use crate::chunks::shared;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MMDX {
    /*
    char filenames[0];              // zero-terminated strings with complete paths to models. Referenced in MMID.
    */
    #[br(parse_with = shared::zero_terminated_strings)]
    pub filenames: Vec<String>,
}