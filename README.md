# Aliexpress SDK Rust

This repository contains a Rust implementation of the Aliexpress SDK. The SDK is designed to handle the complexities of signing requests, managing API parameters, and processing responses. It provides a convenient way to interact with the Aliexpress Open Platform API that require HMAC-SHA256 signing for secure communication.

## Features

- **HMAC-SHA256 Signature Generation**: Automatically generates signatures for API requests using the provided secret key.
- **Request Management**: Facilitates the creation and management of API requests, including adding parameters and handling file uploads.
- **Response Parsing**: Processes API responses and provides structured access to the data.
- **Asynchronous HTTP Requests**: Utilizes asynchronous requests to interact with the API, ensuring non-blocking operations.

## Installation

Add the following to your `Cargo.toml` to include this SDK in your Rust project:

```toml
[dependencies]
aliexpress_sdk = "1.0.0"
```

## Usage

### Creating a Client

Start by creating an `IopClient` with the server URL, `app_key`, and `app_secret`:

```rust
use aliexpress_sdk::IopClient;

let client = IopClient::new("https://api.example.com", "your_app_key", "your_app_secret");
```

### Creating a Request

Create an `IopRequest` by specifying the API method you wish to call. You can add parameters and configure the request format:

```rust
use aliexpress_sdk::IopRequest;

let mut request = IopRequest::new("getProductDetails", "POST");
request.add_api_param("product_id", "12345");
request.set_format("json");
```

### Executing the Request

Execute the request asynchronously using the `IopClient`:

```rust
use tokio::runtime::Runtime;

let runtime = Runtime::new().unwrap();
let response = runtime.block_on(client.execute(&request, None));

match response {
    Ok(resp) => println!("Response: {:?}", resp.body()),
    Err(err) => eprintln!("Error: {}", err),
}
```

### Handling the Response

The SDK returns an `IopResponse` that contains details of the API response. You can access various fields such as the response code, message, and body:

```rust
println!("Code: {:?}", response.code());
println!("Message: {:?}", response.message());
println!("Body: {:?}", response.body());
```

### Example: Signing a Request

The SDK includes a function for generating HMAC-SHA256 signatures required by the API:

```rust
use aliexpress_sdk::sign;
use std::collections::HashMap;

let secret = "your_secret_key";
let api = "/api/getProductDetails";
let mut parameters = HashMap::new();
parameters.insert("product_id".to_string(), "12345".to_string());

let signature = sign(secret, api, &parameters);
println!("Generated Signature: {}", signature);
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.
