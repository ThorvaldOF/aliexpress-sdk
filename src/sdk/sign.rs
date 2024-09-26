use std::collections::HashMap;
use hmac::{KeyInit, Mac};
use crate::sdk::consts::HmacSha256;

pub fn sign(secret: &str, api: &str, parameters: &HashMap<String, String>) -> String {
    let mut sorted_keys: Vec<&String> = parameters.keys().collect();
    sorted_keys.sort();

    let mut parameters_str = String::new();
    if api.contains("/") {
        parameters_str.push_str(api);
    }

    for key in sorted_keys {
        parameters_str.push_str(key);
        parameters_str.push_str(&parameters[key]);
    }

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(parameters_str.as_bytes());
    let result = mac.finalize();

    hex::encode(result.into_bytes()).to_uppercase()
}
