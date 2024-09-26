use chrono::Utc;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::Value;
use crate::iop_request::IopRequest;
use crate::iop_response::IopResponse;
use crate::sign::sign;

pub struct IopClient {
    server_url: String,
    app_key: String,
    app_secret: String,
}

impl IopClient {
    pub fn new(server_url: &str, app_key: &str, app_secret: &str) -> Self {
        Self {
            server_url: server_url.to_string(),
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
        }
    }

    pub async fn execute(&self, request: &IopRequest, access_token: Option<&str>) -> Result<IopResponse, reqwest::Error> {
        let mut sys_parameters = HashMap::new();
        sys_parameters.insert("app_key".to_string(), self.app_key.clone());
        sys_parameters.insert("sign_method".to_string(), "sha256".to_string());
        sys_parameters.insert("timestamp".to_string(), Self::get_timestamp());
        sys_parameters.insert("partner_id".to_string(), "iop-sdk-rust-20220609".to_string());
        sys_parameters.insert("method".to_string(), request.api_name.clone());
        sys_parameters.insert("simplify".to_string(), request.simplify.clone());
        sys_parameters.insert("format".to_string(), request.format.clone());

        if let Some(token) = access_token {
            sys_parameters.insert("session".to_string(), token.to_string());
        }

        let mut sign_parameters = sys_parameters.clone();
        sign_parameters.extend(request.api_params.clone());

        let sign = sign(&self.app_secret, &request.api_name, &sign_parameters);
        sign_parameters.insert("sign".to_string(), sign);

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
        let now = Utc::now();
        now.timestamp_millis().to_string()
    }
}