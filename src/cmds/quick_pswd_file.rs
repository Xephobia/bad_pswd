use std::{
    env::{current_dir, var},
    error::Error,
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub fn quick_pswd_file(create: bool) -> Result<File, Box<dyn Error>> {
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
