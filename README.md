wow_chunky
===========

Overview
-----------

A pure Rust parsing library for World of Warcraft's chunked files, intended to support all TLV-chunked files from versions 1.12 to 3.3.5

Currently assumes that chunks are structured according to 1.12 formats.


Supported formats
------------------

| Format | 1.12 | 2.4.3 | 3.3.5 | Note |
|--------|------|-------|-------|------|
| WDT    | :heavy_check_mark:  | :question:     | :question:     |
| ADT    | :heavy_check_mark:  | :question:     | :question:     | No water (MCLQ) parsing yet.
| BLP (DXT Compressd) | :heavy_check_mark:  | :question:     | :question:     |
| BLP (Other) | :x:  | :x:     | :x:     | PALLETE / ARGB encoded BLPs are unhandled.
| BLS | :x:  | :x:     | :x:     | Heavily corrupted.

Examples
-----------

```rust
    // Load and parse the ADT at (25, 20) in the Azeroth map.
    let wdt_path = PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
    let adt = wow_chunky::parser::adt::ADT::from_wdt_file(wdt_path, 25, 20);
```

```rust
    // Load and parse the WDT first.
    let wdt_path = PathBuf::from("./test_data/Azeroth/Azeroth.wdt");
    let wdt = wow_chunky::parser::wdt::WDT::from_file(wdt_path);

    // Then, using the flags we need from the WDT (required for heightmap parsing), parse the ADT at (31, 30).
    let adt_path = PathBuf::from("./test_data/Azeroth//Azeroth_31_30.adt");
    let mphd_flags = wdt.mphd
        .and_then(|chunk| Some(chunk.flags))
        .expect("WDT should have a valid MPHD chunk");
    let adt = wow_chunky::parser::adt::ADT::from_file(adt_path, mphd_flags)
```
