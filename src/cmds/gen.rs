use std::error::Error;

use rand::{thread_rng, Rng};

pub fn gen(len: usize) -> Result<String, Box<dyn Error>> {
    let rand_bytes = {
        let encode_len = (len * 4 + 2) / 3;
        let mut buf = vec![0u8; encode_len];
        thread_rng().try_fill(buf.as_mut_slice())?;
        buf
    };

    let mut rand_string = base64::encode(rand_bytes);
    rand_string.truncate(len);

    Ok(rand_string)
}
