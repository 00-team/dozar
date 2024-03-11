// use std::sync::OnceLock;

#[derive(Debug)]
/// dozar config
pub struct Config {}

impl Config {
    pub const RECORD_DIR: &'static str = "./records/";
    pub const CODE_ABC: &'static [u8] = b"0123456789";
    pub const TOKEN_ABC: &'static [u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";
}

// pub fn config() -> &'static Config {
//     static STATE: OnceLock<Config> = OnceLock::new();
//     STATE.get_or_init(|| Config {})
// }
