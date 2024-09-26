use hmac::{Hmac};
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

pub(crate) const P_SDK_VERSION: &str = "iop-sdk-rust-20220609";
pub(crate) const P_APPKEY: &str = "app_key";
pub(crate) const P_ACCESS_TOKEN: &str = "session";
pub(crate) const P_TIMESTAMP: &str = "timestamp";
pub(crate) const P_SIGN: &str = "sign";
pub(crate) const P_SIGN_METHOD: &str = "sign_method";
pub(crate) const P_PARTNER_ID: &str = "partner_id";
pub(crate) const P_METHOD: &str = "method";
pub(crate) const P_DEBUG: &str = "debug";
pub(crate) const P_SIMPLIFY: &str = "simplify";
pub(crate) const P_FORMAT: &str = "format";
