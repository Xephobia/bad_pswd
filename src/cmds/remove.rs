use std::{
    error::Error,
    io::{Seek, SeekFrom, Write},
};

use super::{pswd_struct::PswdLst, quick_pswd_file::quick_pswd_file as gen_pswdfile};

pub fn remove(username: String) -> Result<(), Box<dyn Error>> {
    let mut pswd_file = gen_pswdfile(false)?;

    let mut pswd_list = bincode::deserialize_from::<_, PswdLst>(&pswd_file)?;
    pswd_file.seek(SeekFrom::Start(0))?;
    match pswd_list.0.iter().position(|s| s.username == username) {
        Some(before) => pswd_list.0.remove(before),
        None => return Err(format!("{} not found", username).into()),
    };

    let buf = bincode::serialize(&pswd_list)?;
    pswd_file.set_len(buf.len() as u64)?;
    if pswd_file.write(buf.as_slice())? == 0 {
        eprintln!("warning : no bytes could be written");
    }
    pswd_file.flush()?;

    Ok(())
}
