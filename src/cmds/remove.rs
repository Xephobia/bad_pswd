use std::{collections::BTreeMap, error::Error};

use super::quick_pswd_file::quick_pswd_file as gen_pswdfile;

pub fn remove(username: String) -> Result<(), Box<dyn Error>> {
    let pswd_file = gen_pswdfile(false)?;

    let mut pswd_list = bincode::deserialize_from::<_, BTreeMap<String, String>>(&pswd_file)?;
    if let None = pswd_list.remove_entry(&username) {
        return Err(format!("{} was not found", username).into());
    }

    pswd_file.set_len(0)?;
    bincode::serialize_into(pswd_file, &pswd_list)?;

    Ok(())
}
