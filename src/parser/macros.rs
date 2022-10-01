#[macro_export]
macro_rules! parse_chunk {
    ( $chunk_type:ty, $data:expr, &mut $ref:expr) => {
        {
            let chunk = parse_chunk_data::<$chunk_type>($data)?;
            $ref = Some(chunk);
        }
    };
}

pub(crate) use parse_chunk;
