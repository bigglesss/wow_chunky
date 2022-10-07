//! A pure Rust parsing library for World of Warcraft's chunked files (and some additional binary files), targeting versions 1.12 to 3.3.5
//! 
//! Supported formats
//! ------------------
//! 
//! | Format | 1.12 | 2.4.3 | 3.3.5 | Note |
//! |--------|------|-------|-------|------|
//! | WDT    | ✔  | ?     | ?     |
//! | ADT    | ✔  | ?     | ?     | 
//! | BLP (DXT Compressed) | ✔  | ?     | ?     |
//! | BLP (Other) | ✖  | ✖     | ✖     | PALLETE / ARGB encoded BLPs are unhandled.
//! | BLS | ✖  | ✖     | ✖     | Heavily corrupted.
//!
//! Examples
//! -----------
//!
//! 
//! ```rust
//! // Load and parse the ADT at (25, 20) in the Azeroth map.
//! let wdt_path = std::path::PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
//! let adt = wow_chunky::files::ADT::from_wdt_file(wdt_path, 25, 20);
//! ```
//! 
//! ```rust
//! // Load and parse the WDT first.
//! let wdt_path = std::path::PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
//! let wdt = wow_chunky::files::WDT::from_file(wdt_path).expect("Invalid WDT file");
//! 
//! // Then, using the flags we need from the WDT (required for heightmap parsing),
//! // parse the ADT at (31, 30).
//! let adt = wow_chunky::files::ADT::from_wdt(&wdt, 31, 30);
//! ```

pub mod chunks;
pub mod files;
pub mod error;