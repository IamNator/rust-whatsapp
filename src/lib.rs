pub mod calls;

use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

const DEFAULT_API_URL: &str = "https://api.whatsapp.com";
const DEFAULT_RATE_LIMIT: usize = 200;
const DEFAULT_TIMEOUT: u64 = 3;

#[derive(Debug, Serialize, Deserialize)]
struct APIErrorData {
    details: String,
    messaging_product: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct APIError {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    code: i32,
    error_data: APIErrorData,
    error_subcode: i32,
    fbtrace_id: String,
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct APIResponseContact {
    input: String,
    wa_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct APIResponseMessage {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct APIResponse {
    error: Option<APIError>,
    messaging_product: String,
    contacts: Vec<APIResponseContact>,
    messages: Vec<APIResponseMessage>,
}

impl APIResponse {
    fn is_successful(&self) -> bool {
        self.error.is_none()
    }
}

#[derive(Debug, Serialize)]
struct APIRequest<'a> {
    messaging_product: &'static str,
    to: &'a str,
    #[serde(rename = "type")]
    payload_type: PayloadType,
    text: Option<Text>,
    template: Option<Template>,
}

impl<'a> APIRequest<'a> {
    fn new_text(to: &'a str, body: &'a str) -> Self {
        APIRequest {
            messaging_product: "whatsapp",
            to,
            payload_type: PayloadType::Text,
            text: Some(Text { body }),
            template: None,
        }
    }

    fn new_template(to: &'a str, template: Template) -> Self {
        APIRequest {
            messaging_product: "whatsapp",
            to,
            payload_type: PayloadType::Template,
            text: None,
            template: Some(template),
        }
    }
}

#[derive(Debug, Serialize)]
enum PayloadType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "template")]
    Template,
}

#[derive(Debug, Serialize)]
struct Text<'a> {
    body: &'a str,
}

struct APICaller {
    client: Client,
}

impl APICaller {
    async fn post(
        &self,
        url: &str,
        body: Vec<u8>,
        headers: HashMap<&str, &str>,
    ) -> Result<APIResponse, reqwest::Error> {
        let response = self
            .client
            .post(url)
            .body(body)
            .headers(headers)
            .send()
            .await?;

        let status_code = response.status();
        let api_response: APIResponse = response.json().await?;

        if status_code.is_success() {
            Ok(api_response)
        } else {
            Err(reqwest::Error::new(
                reqwest::Error::Status(status_code),
                Some(Box::new(api_response)),
            ))
        }
    }
}

struct Client<'a> {
    phone_number_id: &'a str,
    access_token: &'a str,
    base_url: &'a str,
    api_version: APIVersion,
    api_caller: APICaller,
    debug: bool,
}

impl<'a> Client<'a> {
    fn new(phone_number_id: &'a str, access_token: &'a str, opts: &[Opt]) -> Self {
        let base_url = DEFAULT_API_URL;
        let api_version = APIVersion::V15;
        let api_caller = APICaller {
            client: Client::new(phone_number_id, access_token, opts),
        };

        let mut client = Client {
            phone_number_id,
            access_token,
            base_url,
            api_version,
            api_caller,
            debug: false,
        };

        for opt in opts {
            opt(&mut client);
        }

        client
    }

    fn set_base_url(&mut self, base_url: &'a str) {
        self.base_url = base_url;
    }

    fn set_api_version(&mut self, api_version: APIVersion) {
        self.api_version = api_version;
    }

    fn set_api_caller(&mut self, api_caller: APICaller) {
        self.api_caller = api_caller;
    }

    async fn send(&self, to: &'a str, msg: APIRequest<'a>) -> Result<APIResponse, reqwest::Error> {
        let url = format!("{}/{}/messages", self.base_url, self.api_version);
        let mut headers = HashMap::new();
        headers.insert("Authorization", &format!("Bearer {}", self.access_token));
        headers.insert("Content-Type", "application/json");
        headers.insert("Accept", "application/json");

        let body = serde_json::to_vec(&msg)?;

        self.api_caller.post(&url, body, headers).await
    }

    async fn send_text(&self, to: &'a str, text: &'a str) -> Result<APIResponse, reqwest::Error> {
        let msg = APIRequest::new_text(to, text);
        self.send(to, msg).await
    }

    async fn send_template(
        &self,
        to: &'a str,
        template: Template,
    ) -> Result<APIResponse, reqwest::Error> {
        let msg = APIRequest::new_template(to, template);
        self.send(to, msg).await
    }
}

type Opt<'a> = Box<dyn FnOnce(&mut Client<'a>)>;

fn with_base_url<'a>(url: &'a str) -> Opt<'a> {
    Box::new(move |client| client.set_base_url(url))
}

fn with_api_version<'a>(api_version: APIVersion) -> Opt<'a> {
    Box::new(move |client| client.set_api_version(api_version))
}

fn with_api_caller<'a>(api_caller: APICaller) -> Opt<'a> {
    Box::new(move |client| client.set_api_caller(api_caller))
}
