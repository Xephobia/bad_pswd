use std::{
    collections::HashMap,
    env::{current_dir, var},
    error::Error,
    fs::{File, OpenOptions},
    path::PathBuf,
};

fn quick_pswd_file(create: bool) -> Result<File, Box<dyn Error>> {
    let pswd_path = match var("PSWD_FILE") {
        Ok(p) => PathBuf::from(p),
        Err(std::env::VarError::NotPresent) => {
            let mut d = current_dir()?;
            d.push("pswdlist");
            d
        }
        Err(e) => return Err(e.into()),
    };

    let pswd_file = OpenOptions::new()
        .write(true)
        .create(create)
        .read(true)
        .open(pswd_path)?;

    Ok(pswd_file)
}

pub fn add(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let username = match args.get(2) {
        Some(username) => username,
        None => return Err("username excepted".into()),
    };
    let pswd = match args.get(3) {
        Some(pswd) => pswd,
        None => return Err("password excepted".into()),
    };

    let pswd_file = quick_pswd_file(true)?;
    pswd_file.sync_all()?;
    let mut pswd_list: HashMap<String, String> = match pswd_file.metadata()?.len() {
        0 => HashMap::new(),
        _ => bincode::deserialize_from(&pswd_file)?,
    };
    if let Some(before) = pswd_list.insert(username.to_owned(), pswd.to_owned()) {
        println!("warning : password changed, old value : {}", before);
    }
    pswd_file.set_len(0)?;
    bincode::serialize_into(pswd_file, &pswd_list)?;

    Ok(())
}
