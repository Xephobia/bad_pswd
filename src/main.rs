use std::env::args;

static USAGE: &str = "usage :\n
    -add [username] [pswd] : add password using username [key] and password [value]\n
    -remove [username] : removes password matching username\n
    -get [username] : get a password matching username\n
    -gen : generates a random password";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = args().collect();

    match args.get(1).map(String::as_str) {
        None => return Err(USAGE.into()),
        Some("-add") => {}
        Some("-remove") => {}
        Some("-gen") => {}
        Some(other) => {}
    }

    Ok(())
}
