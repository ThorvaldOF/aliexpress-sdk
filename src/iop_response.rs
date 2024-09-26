use serde_json::Value;

pub struct IopResponse {
    pub(crate) r_type: Option<String>,
    code: Option<String>,
    message: Option<String>,
    request_id: Option<String>,
    pub(crate) body: Option<Value>,
}

impl IopResponse {
    pub fn new() -> Self {
        Self {
            r_type: None,
            code: None,
            message: None,
            request_id: None,
            body: None,
        }
    }

    pub fn from_json(json: Value) -> Self {
        let mut response = IopResponse::new();
        if let Some(code) = json.get("code") {
            response.code = code.as_str().map(|s| s.to_string());
        }
        if let Some(r_type) = json.get("type") {
            response.r_type = r_type.as_str().map(|s| s.to_string());
        }
        if let Some(message) = json.get("message") {
            response.message = message.as_str().map(|s| s.to_string());
        }
        if let Some(request_id) = json.get("request_id") {
            response.request_id = request_id.as_str().map(|s| s.to_string());
        }
        response.body = Some(json);
        response
    }
    pub fn r_type(&self) -> Option<&String> {
        self.r_type.as_ref()
    }
    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
    pub fn request_id(&self) -> Option<&String> {
        self.request_id.as_ref()
    }
    pub fn body(&self) -> Option<&Value> {
        self.body.as_ref()
    }
}