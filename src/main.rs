use std::env::args;

mod cmds;
use cmds::add;

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
            add::add(&args)?;
        }
        Some("-remove") => {}
        Some("-gen") => {}
        Some(other) => {}
    }

    Ok(())
}
