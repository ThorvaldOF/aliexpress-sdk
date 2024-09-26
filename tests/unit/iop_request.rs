use aliexpress_sdk::iop_request::IopRequest;

#[test]
fn test_iop_request_new() {
    let api_name = "getProduct";
    let http_method = "POST";
    let request = IopRequest::new(api_name, http_method);

    assert_eq!(request.api_name(), api_name);
    assert_eq!(request.http_method(), http_method);
    assert!(request.api_params().is_empty());
    assert!(request.file_params().is_empty());
    assert_eq!(request.simplify(), "false");
    assert_eq!(request.format(), "json");
}

#[test]
fn test_add_api_param() {
    let mut request = IopRequest::new("getProduct", "POST");
    request.add_api_param("product_id", "12345");

    assert_eq!(request.api_params().get("product_id"), Some(&"12345".to_string()));
    assert_eq!(request.api_params().len(), 1);
}

#[test]
fn test_add_file_param() {
    let mut request = IopRequest::new("uploadImage", "POST");
    request.add_file_param("image", "image_data");

    assert_eq!(request.file_params().get("image"), Some(&"image_data".to_string()));
    assert_eq!(request.file_params().len(), 1);
}

#[test]
fn test_set_simplify() {
    let mut request = IopRequest::new("getProduct", "POST");
    request.set_simplify();

    assert_eq!(request.simplify(), "true");
}

#[test]
fn test_set_format() {
    let mut request = IopRequest::new("getProduct", "POST");
    request.set_format("xml");

    assert_eq!(request.format(), "xml");
}

#[test]
fn test_combined_operations() {
    let mut request = IopRequest::new("getProduct", "POST");

    request.add_api_param("product_id", "12345");
    request.add_api_param("category", "electronics");
    request.add_file_param("image", "image_data");
    request.set_simplify();
    request.set_format("xml");

    assert_eq!(request.api_name(), "getProduct");
    assert_eq!(request.http_method(), "POST");
    assert_eq!(request.api_params().get("product_id"), Some(&"12345".to_string()));
    assert_eq!(request.api_params().get("category"), Some(&"electronics".to_string()));
    assert_eq!(request.file_params().get("image"), Some(&"image_data".to_string()));
    assert_eq!(request.simplify(), "true");
    assert_eq!(request.format(), "xml");
}
