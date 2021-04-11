use std::{
    error::Error,
    io::{Seek, SeekFrom, Write},
};

use super::{
    pswd_struct::{PswdLst, UsrPswd},
    quick_pswd_file::quick_pswd_file as gen_pswdfile,
};

pub fn add(username: String, pswd: String) -> Result<(), Box<dyn Error>> {
    let mut pswd_file = gen_pswdfile(true)?;
    pswd_file.sync_all()?;
    let mut pswd_list: PswdLst = match pswd_file.metadata()?.len() {
        0 => PswdLst(Vec::new()),
        _ => {
            let deserialized = bincode::deserialize_from(&pswd_file)?;
            pswd_file.seek(SeekFrom::Start(0))?;
            deserialized
        }
    };
    match pswd_list.0.iter().position(|s| s.username == username) {
        Some(before) => {
            println!(
                "warning : password changed, old value : {}",
                pswd_list.0[before].password
            );
            pswd_list.0[before].password = pswd;
        }
        None => pswd_list.0.push(UsrPswd {
            username,
            password: pswd,
        }),
    }

    let buf = bincode::serialize(&pswd_list)?;
    pswd_file.set_len(buf.len() as u64)?;
    if pswd_file.write(buf.as_slice())? == 0 {
        eprintln!("warning : no bytes could be written");
    }
    pswd_file.flush()?;

    Ok(())
}
