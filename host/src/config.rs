//shell of choose, please use full path if an error occurs otherwise just use shell name.
pub const SHELL: &str = "zsh";
//if you want host to log and store ran commands
pub const LOG: bool = true;
//public name of the server
pub const NAME: &str = "mac";
//encryption settings
pub const ENCRYPTION: crate::db::Encrypt =  crate::db::Encrypt{
    //your encryption key
    key : "hello",
    //if you want encryption
    encryption_on : true
};
// {uri_scheme}://:{redis_password}@{redis_host_name}
pub const REDIS_KEY: &str  = "";
// If you want sentry_logging logging information put your key here
pub const SENTRY: crate::db::sentry_logging::Sentry = crate::db::sentry_logging::Sentry {
    //your key goes here
    key : "your_sentry_key",
    // if you want it make it true otherwise let it remain at false
    is_on : false
};
