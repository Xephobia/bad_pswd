use std::{collections::BTreeMap, error::Error};

use super::quick_pswd_file::quick_pswd_file as gen_pswdfile;

pub fn add(username: String, pswd: String) -> Result<(), Box<dyn Error>> {
    let pswd_file = gen_pswdfile(true)?;
    pswd_file.sync_all()?;
    let mut pswd_list: BTreeMap<String, String> = match pswd_file.metadata()?.len() {
        0 => BTreeMap::new(),
        _ => bincode::deserialize_from(&pswd_file)?,
    };
    if let Some(before) = pswd_list.insert(username, pswd) {
        println!("warning : password changed, old value : {}", before);
    }
    pswd_file.set_len(0)?;
    bincode::serialize_into(pswd_file, &pswd_list)?;

    Ok(())
}