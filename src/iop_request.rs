use std::collections::HashMap;

pub struct IopRequest {
    pub(crate) api_name: String,
    pub(crate) api_params: HashMap<String, String>,
    pub(crate) file_params: HashMap<String, String>,
    pub(crate) simplify: String,
    pub(crate) format: String,
    pub(crate) http_method: String,
}

impl IopRequest {
    pub fn new(api_name: &str, http_method: &str) -> Self {
        Self {
            api_name: api_name.to_string(),
            api_params: HashMap::new(),
            file_params: HashMap::new(),
            simplify: "false".to_string(),
            format: "json".to_string(),
            http_method: http_method.to_string(),
        }
    }

    pub fn add_api_param(&mut self, key: &str, value: &str) {
        self.api_params.insert(key.to_string(), value.to_string());
    }

    pub fn add_file_param(&mut self, key: &str, value: &str) {
        self.file_params.insert(key.to_string(), value.to_string());
    }

    pub fn set_simplify(&mut self) {
        self.simplify = "true".to_string();
    }

    pub fn set_format(&mut self, value: &str) {
        self.format = value.to_string();
    }

    pub fn api_name(&self) -> &str {
        &self.api_name
    }

    pub fn api_params(&self) -> &HashMap<String, String> {
        &self.api_params
    }

    pub fn file_params(&self) -> &HashMap<String, String> {
        &self.file_params
    }

    pub fn simplify(&self) -> &str {
        &self.simplify
    }

    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn http_method(&self) -> &str {
        &self.http_method
    }
}