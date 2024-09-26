use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::Client;
use serde_json::Value;
use crate::consts::{P_ACCESS_TOKEN, P_APPKEY, P_FORMAT, P_METHOD, P_PARTNER_ID, P_SDK_VERSION, P_SIGN, P_SIGN_METHOD, P_SIMPLIFY, P_TIMESTAMP};
use crate::iop_request::IopRequest;
use crate::iop_response::IopResponse;
use crate::sign::sign;

pub struct IopClient {
    server_url: String,
    app_key: String,
    app_secret: String,
    timeout: u64,
}

impl IopClient {
    pub fn new(server_url: &str, app_key: &str, app_secret: &str, timeout: u64) -> Self {
        Self {
            server_url: server_url.to_string(),
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
            timeout,
        }
    }

    pub async fn execute(&self, request: &IopRequest, access_token: Option<&str>) -> Result<IopResponse, reqwest::Error> {
        let mut sys_parameters = HashMap::new();
        sys_parameters.insert(P_APPKEY.to_string(), self.app_key.clone());
        sys_parameters.insert(P_SIGN_METHOD.to_string(), "sha256".to_string());
        sys_parameters.insert(P_TIMESTAMP.to_string(), Self::get_timestamp());
        sys_parameters.insert(P_PARTNER_ID.to_string(), P_SDK_VERSION.to_string());
        sys_parameters.insert(P_METHOD.to_string(), request.api_name.clone());
        sys_parameters.insert(P_SIMPLIFY.to_string(), request.simplify.clone());
        sys_parameters.insert(P_FORMAT.to_string(), request.format.clone());

        if let Some(token) = access_token {
            sys_parameters.insert(P_ACCESS_TOKEN.to_string(), token.to_string());
        }

        let mut sign_parameters = sys_parameters.clone();
        sign_parameters.extend(request.api_params.clone());

        let sign = sign(&self.app_secret, &request.api_name, &sign_parameters);
        sign_parameters.insert(P_SIGN.to_string(), sign);

        let client = Client::new();
        let full_url = format!("{}?", self.server_url);

        let res = if request.http_method == "POST" || !request.file_params.is_empty() {
            client.post(&full_url)
                .json(&sign_parameters)
                .send().await?
        } else {
            client.get(&full_url)
                .query(&sign_parameters)
                .send().await?
        };

        let json: Value = res.json().await?;
        Ok(IopResponse::from_json(json))
    }

    pub fn get_timestamp() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        (since_the_epoch.as_secs() * 1000).to_string()
    }
}