//! A pure Rust parsing library for World of Warcraft's chunked files, intended to support all TLV-chunked files from versions 1.12 to 3.3.5
//! 
//! Supported formats
//! ------------------
//! 
//! | Format | 1.12 | 2.4.3 | 3.3.5 | Note |
//! |--------|------|-------|-------|------|
//! | WDT    | ✔  | ?     | ?     |
//! | ADT    | ✔  | ?     | ?     | No water (MCLQ) parsing yet.
//! | BLP (DXT Compressd) | ✔  | ?     | ?     |
//! | BLP (Other) | ✖  | ✖     | ✖     | PALLETE / ARGB encoded BLPs are unhandled.
//! | BLS | ✖  | ✖     | ✖     | Heavily corrupted.
//!
//! Examples
//! -----------
//! 
//! ```rust
//!     // Load and parse the ADT at (25, 20) in the Azeroth map.
//!     let wdt_path = PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
//!     let adt = wow_chunky::parser::adt::ADT::from_wdt_file(wdt_path, 25, 20);
//! ```
//! 
//! ```rust
//!     // Load and parse the WDT first.
//!     let wdt_path = PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
//!     let wdt = wow_chunky::parser::wdt::WDT::from_file(wdt_path);
//! 
//!     // Then, using the flags we need from the WDT (required for heightmap parsing), parse the ADT at (31, 30).
//!     let adt = wow_chunky::parser::adt::ADT::from_wdt(wdt, 31, 30)
//! ```

pub mod chunks;
pub mod files;
pub mod error;