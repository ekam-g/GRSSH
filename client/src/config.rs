// {uri_scheme}://:{redis_password}@{redis_host_name}
pub const REDIS_KEY: &str  = "";
//encryption settings
pub const ENCRYPTION: crate::db::Encrypt =  crate::db::Encrypt{
    //your encryption key
    key : "hello",
    //if you want encryption
    encryption_on : true
};