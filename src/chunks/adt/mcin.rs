use binread::BinRead;

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MCIN {
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
