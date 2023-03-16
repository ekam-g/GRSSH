// redis://<user>:<password>@<endpoint>:<port>
pub const REDIS_KEY: &str  = "redis://127.0.0.1/";
//encryption settings
pub const ENCRYPTION: crate::db::Encrypt =  crate::db::Encrypt{
    //your encryption key
    key : "hello",
    //if you want encryption
    encryption_on : true
};