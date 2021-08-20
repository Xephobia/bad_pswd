# bad password manager, supposed bad.

Usage : `bad_pswd.exe [option] (arguments)`
options :
* `-add` add password using username and password
arguments : *username* *pswd*
* `-remove` remove password matching username
arguments : *username*
* `-get` gets password matching username
arguments : *username*
* `-gen` generates password of specified length
arguments : *length*

## where are the password stored?

This is a bad password manager, so by no mean the data will be secured. All the passwords are stored in file pointed by the environement variable `PSWD_FILE` or in `WORKING_DIR/pswdlist` (you know what `WORKING_DIR` represent)

### how the data is stored?

Well, you can always inspect the code but i could save you some time : the data is serialized using the crate `bincode` each username/password pair is represented by this struct

```rust
pub struct UsrPswd {
    pub username: String,
    pub password: String,
}
```

the whole list is just
```rust
struct PswdLst(pub Vec<UsrPswd>);
```

maybe i will change it to a `BTreeMap<String, String>` one day, dunno