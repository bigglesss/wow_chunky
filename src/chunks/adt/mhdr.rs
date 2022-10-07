use binread::BinRead;

#[derive(Clone, Debug, BinRead)]
#[br(little, repr = u32)]
pub enum MHDRFlags {
    NONE = 0,
    MFBO = 1,
    NORTHREND = 2,
}

#[derive(Clone, Debug, BinRead)]
#[br(little)]
pub struct MHDR {
    /*
    enum MHDRFlags {
        mhdr_MFBO = 1,                // contains a MFBO chunk.
        mhdr_northrend = 2,           // is set for some northrend ones.
    };
    uint32_t flags;
    uint32_t mcin;                 // MCIN*, Cata+: obviously gone. probably all offsets gone, except mh2o(which remains in root file).
    uint32_t mtex;                 // MTEX*
    uint32_t mmdx;                 // MMDX*
    uint32_t mmid;                 // MMID*
    uint32_t mwmo;                 // MWMO*
    uint32_t mwid;                 // MWID*
    uint32_t mddf;                 // MDDF*
    uint32_t modf;                 // MODF*
    uint32_t mfbo;                 // MFBO*   this is only set if flags & mhdr_MFBO.
    uint32_t mh2o;                 // MH2O*
    uint32_t mtxf;                 // MTXF*
    uint8_t mamp_value;             // Cata+, explicit MAMP chunk overrides data
    uint8_t padding[3];
    uint32_t unused[3];
    */
    pub flags: MHDRFlags,
}