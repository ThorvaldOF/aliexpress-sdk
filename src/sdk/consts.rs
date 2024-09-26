use hmac::{Hmac};
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

//TODO: en python c'est des variables
pub const P_SDK_VERSION: &str = "iop-sdk-rust-20220609";
pub const P_APPKEY: &str = "app_key";
pub const P_ACCESS_TOKEN: &str = "session";
pub const P_TIMESTAMP: &str = "timestamp";
pub const P_SIGN: &str = "sign";
pub const P_SIGN_METHOD: &str = "sign_method";
pub const P_PARTNER_ID: &str = "partner_id";
pub const P_METHOD: &str = "method";
pub const P_DEBUG: &str = "debug";
pub const P_SIMPLIFY: &str = "simplify";
pub const P_FORMAT: &str = "format";
