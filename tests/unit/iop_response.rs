use serde_json::json;
use aliexpress_sdk::iop_response::IopResponse;

#[test]
fn test_iop_response_new() {
    let response = IopResponse::new();

    assert!(response.r_type().is_none());
    assert!(response.code().is_none());
    assert!(response.message().is_none());
    assert!(response.request_id().is_none());
    assert!(response.body().is_none());
}

#[test]
fn test_iop_response_from_json_full() {
    let json_data = json!({
        "code": "200",
        "type": "success",
        "message": "Request successful",
        "request_id": "abc123",
        "data": { "key": "value" }
    });

    let response = IopResponse::from_json(json_data.clone());

    assert_eq!(response.code(), Some(&"200".to_string()));
    assert_eq!(response.r_type(), Some(&"success".to_string()));
    assert_eq!(response.message(), Some(&"Request successful".to_string()));
    assert_eq!(response.request_id(), Some(&"abc123".to_string()));
    assert_eq!(response.body(), Some(&json_data));
}

#[test]
fn test_iop_response_from_json_partial() {
    let json_data = json!({
        "code": "404",
        "message": "Not Found",
    });

    let response = IopResponse::from_json(json_data.clone());

    assert_eq!(response.code(), Some(&"404".to_string()));
    assert_eq!(response.r_type(), None);
    assert_eq!(response.message(), Some(&"Not Found".to_string()));
    assert_eq!(response.request_id(), None);
    assert_eq!(response.body(), Some(&json_data));
}

#[test]
fn test_iop_response_from_json_empty() {
    let json_data = json!({});

    let response = IopResponse::from_json(json_data.clone());

    assert!(response.code().is_none());
    assert!(response.r_type().is_none());
    assert!(response.message().is_none());
    assert!(response.request_id().is_none());
    assert_eq!(response.body(), Some(&json_data));
}

#[test]
fn test_iop_response_handles_non_string_fields() {
    let json_data = json!({
        "code": 200,
        "message": "OK",
        "request_id": 12345,
    });

    let response = IopResponse::from_json(json_data.clone());

    assert!(response.code().is_none());
    assert!(response.request_id().is_none());
    assert_eq!(response.message(), Some(&"OK".to_string()));
    assert_eq!(response.body(), Some(&json_data));
}
