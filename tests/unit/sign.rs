use std::collections::HashMap;
use aliexpress_sdk::sign::sign;

#[test]
fn test_sign_basic_case() {
    let secret = "my_secret";
    let api = "/api/v1/test";
    let mut parameters = HashMap::new();
    parameters.insert("key1".to_string(), "value1".to_string());
    parameters.insert("key2".to_string(), "value2".to_string());

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "6F4B2F051F32C0B6348C04DEEADB6993A1F10C2424E11EC9DF9031C45FDD9008");
}

#[test]
fn test_sign_with_empty_parameters() {
    let secret = "my_secret";
    let api = "/api/v1/test";
    let parameters = HashMap::new();

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "3356228AB8C4CCE0B867FD40E401A70AA8A3481A359E070A92A97EA52200787B");
}

#[test]
fn test_sign_without_api_path() {
    let secret = "my_secret";
    let api = "";
    let mut parameters = HashMap::new();
    parameters.insert("key1".to_string(), "value1".to_string());

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "CD6F8F7EBA0C7B79DEA9C5DDD5F5738CDAD9CB75870E8E8E3ECA3C8FC2E02058");
}

#[test]
fn test_sign_with_special_characters_in_parameters() {
    let secret = "my_secret";
    let api = "/api/v1/test";
    let mut parameters = HashMap::new();
    parameters.insert("key1".to_string(), "value@1!".to_string());
    parameters.insert("key2".to_string(), "value#2$".to_string());

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "1E0715BD078EA684890CBAACA0B1E4B0721D764641EA40F0C506E14A3282DA85");
}

#[test]
fn test_sign_with_different_order_of_parameters() {
    let secret = "my_secret";
    let api = "/api/v1/test";
    let mut parameters1 = HashMap::new();
    parameters1.insert("key1".to_string(), "value1".to_string());
    parameters1.insert("key2".to_string(), "value2".to_string());

    let mut parameters2 = HashMap::new();
    parameters2.insert("key2".to_string(), "value2".to_string());
    parameters2.insert("key1".to_string(), "value1".to_string());

    let signature1 = sign(secret, api, &parameters1);
    let signature2 = sign(secret, api, &parameters2);

    // Les signatures doivent être identiques car l'ordre des paramètres ne doit pas affecter le résultat
    assert_eq!(signature1, signature2);
}

#[test]
fn test_sign_with_empty_api_and_parameters() {
    let secret = "my_secret";
    let api = "";
    let parameters = HashMap::new();

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "818D1A1458FF2B6F150C6A7E3D839C595F8AC23AA01A8412B1D558D34E7961D2");
}

#[test]
fn test_sign_with_long_secret_and_parameters() {
    let secret = "a_very_long_secret_key_that_should_be_tested_for_hmac_generation";
    let api = "/api/v1/test";
    let mut parameters = HashMap::new();
    parameters.insert("key1".to_string(), "value1".to_string());
    parameters.insert("key2".to_string(), "value2".to_string());

    let signature = sign(secret, api, &parameters);

    assert_eq!(signature, "5A5738868288D223644D72CBFD326E7003875D5DA2D5492963C08F1D15F1A839");
}
