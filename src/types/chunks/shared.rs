use core::fmt::Debug;
use std::io::{Read, Seek};
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

#[derive(Debug, BinRead)]
#[br(little)]
pub struct CAaBox {
    min: C3Vector,
    max: C3Vector,
}

#[derive(Clone, Copy, Debug, BinRead)]
#[br(little)]
pub struct C3Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn char_vec_to_string_le(v: &Vec<u8>, reversed: bool) -> String {
    if reversed {
        v.iter().rev().map(|v| char::from(*v)).collect::<String>()
    } else {
        v.iter().map(|v| char::from(*v)).collect::<String>()
    }
}


pub fn zero_terminated_strings<R: Read + Seek>(
    reader: &mut R,
    _: &ReadOptions,
    _: (),
) -> BinResult<Vec<String>> {
    let mut strings: Vec<String> = Vec::new();
    let mut string_buf: Vec<u8> = Vec::new();

    loop {
        match reader.read_le::<u8>() {
            Ok(v) => {
                if v != u8::MIN {
                    string_buf.push(v);
                } else {
                    strings.push(char_vec_to_string_le(&string_buf, false));
                    string_buf.clear();
                }
            }
            Err(_) => break,
        }
    }

    Ok(strings)
}

pub fn read_until_end<R: Read + Seek, T: BinRead>(
    reader: &mut R,
    _: &ReadOptions,
    _: (),
) -> BinResult<Vec<T>> {
    let mut values: Vec<T> = Vec::new();

    loop {
        match reader.read_le::<T>() {
            Ok(v) => {
                values.push(v);
            }
            Err(_) => break,
        }
    }

    Ok(values)
}

pub fn token_parse<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ()) -> BinResult<String> {
    // Read 4 u8s into a buffer.
    let mut token = vec![0; 4];
    reader.read_exact(&mut token)?;

    // Parse by reversing the buffer and converting to a String.
    let string = char_vec_to_string_le(&token, true);

    Ok(string)
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct ChunkWrapper {
    #[br(parse_with = token_parse)]
    pub token: String,
    pub size: u32,
    #[br(count = size)]
    pub data: Vec<u8>,
}
