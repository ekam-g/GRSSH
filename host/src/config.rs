//shell of choose, please use full path if an error occurs otherwise just use shell name.
pub const SHELL: &str = "zsh";
//if you want host to log and store ran commands
pub const LOG: bool = false;
//public name of the server
pub const NAME: &str = "fedora";
//redis key stored in file
pub const LOCATION_TO_REDIS_KEY: &str = "redis_key.txt";
//encryption settings
pub const ENCRYPTION: crate::db::Encrypt =  crate::db::Encrypt{
    //your encryption key
    key : "hello",
    //if you want encryption
    encryption_on : false
};
