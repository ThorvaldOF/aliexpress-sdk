# IOP SDK Rust

This project is a Rust port of an SDK originally written in Python. The SDK simplifies interactions with an API by managing HMAC signature generation, organizing parameters, and executing HTTP requests. The SDK is designed to be flexible, easy to use, and production-ready.

## Features

- **HMAC Signature Generation**: The SDK uses a secret key to generate HMAC-SHA256 signatures to secure API requests.
- **API Parameter Management**: The SDK facilitates the addition and management of API parameters and files for POST requests.
- **Asynchronous HTTP Requests**: The SDK handles GET and POST requests using the `reqwest` library, supporting asynchronous requests for better performance.
- **Logging**: The SDK includes a logging mechanism to record API errors, with configurable logging levels.
- **Ease of Use**: The SDK simplifies authentication of requests by automatically adding timestamps, signatures, and managing access tokens.

## Installation

Add the following line to your `Cargo.toml` to include this SDK in your project:

```toml
[dependencies]
iop-sdk = { git = "https://github.com/your-repo/iop-sdk-rust" }
```

## Usage

### Creating a Client

To start using the SDK, first create an `IopClient` with the server URL, `app_key`, `app_secret`, and a timeout:

```rust
use iop_sdk::IopClient;

let client = IopClient::new("https://api.example.com", "your_app_key", "your_app_secret", 30);
```

### Creating a Request

Next, create a request by specifying the API name you want to call:

```rust
use iop_sdk::IopRequest;

let mut request = IopRequest::new("getProductDetails", "POST");
request.add_api_param("product_id", "12345");
request.set_format("json");
```

### Executing the Request

Once the request is configured, you can execute it using the client:

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

The SDK returns an `IopResponse` structure that contains the API response details:

```rust
println!("Code: {:?}", response.code());
println!("Message: {:?}", response.message());
println!("Body: {:?}", response.body());
```

## Logging

The SDK includes a logging mechanism to record errors encountered during API calls. By default, the logging level is set to `ERROR`, but you can modify it according to your needs.

## Contribution

Contributions are welcome! Please submit pull requests and issues on the GitHub repository.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
