wow_chunky
===========

Overview
-----------

A pure rust parsing library for World of Warcraft's chunked files, intended to support all TLV-chunked files from versions 1.12 to 3.3.5

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