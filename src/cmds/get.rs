use super::{pswd_struct::PswdLst, quick_pswd_file::quick_pswd_file as gen_pswdfile};
use std::error::Error;

pub fn get(username: String) -> Result<String, Box<dyn Error>> {
    let pswd_file = gen_pswdfile(false)?;
    match bincode::deserialize_from::<_, PswdLst>(pswd_file)?
        .0
        .iter()
        .find(|u| u.username == username)
    {
        Some(p) => Ok(p.password.to_owned()),
        None => Err(format!("{} not found", username).into()),
    }
}
