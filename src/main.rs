use std::env::args;

mod cmds;
use cmds::{add, gen, get, remove};

static USAGE: &str = r#"usage :
-add [username] [pswd] : add password using username [key] and password [value]
-remove [username] : removes password matching username
-get [username] : get a password matching username
-gen : generates a random password"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = args().collect();

    match args.get(1).map(String::as_str) {
        None => {
            eprintln!("{}", USAGE);
            std::process::exit(1)
        }
        Some("-add") => {
            let username = match args.get(2) {
                Some(u) => u.to_owned(),
                None => return Err("username excepted".into()),
            };
            let pswd = match args.get(3) {
                Some(p) => p.to_owned(),
                None => return Err("password excepted".into()),
            };

            add::add(username, pswd)?;
        }
        Some("-remove") => {
            let username = match args.get(2) {
                Some(u) => u.to_owned(),
                None => return Err("username excepted".into()),
            };

            remove::remove(username)?;
        }
        Some("-get") => {
            let username = match args.get(2) {
                Some(u) => u.to_owned(),
                None => return Err("username excepted".into()),
            };
            println!("{}", get::get(username)?);
        }
        Some("-gen") => {
            let len: usize = match args.get(2) {
                Some(l) => l.parse()?,
                None => return Err("length excepted".into()),
            };

            println!("{}", gen::gen(len)?);
        }
        Some(other) => return Err(format!("{} option not found", other).into()),
    }

    Ok(())
}
