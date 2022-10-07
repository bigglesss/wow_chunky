use core::fmt::Debug;

use binread::BinRead;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MAINTile {
    pub has_adt: u32,
    pub flag_loaded: u32,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MAIN {
    #[br(count = 4096)]
    pub tiles: Vec<MAINTile>,
}
