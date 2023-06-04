use reqwest::{Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;

pub trait APICaller {
    fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<(T, u16), Box<dyn Error>>;
}

pub struct Caller {
    client: Client,
}

impl APICaller for Caller {
    fn post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: &[u8],
        headers: HashMap<&str, &str>,
    ) -> Result<(T, u16), Box<dyn Error>> {
        let request = self.client.post(url).body(body);
        let request = attach_headers(request, headers);

        let response = request.send()?;
        let status_code = response.status().as_u16();

        let api_response: T = response.json()?;
        Ok((api_response, status_code))
    }
}

fn attach_headers(request: RequestBuilder, headers: HashMap<&str, &str>) -> RequestBuilder {
    headers
        .iter()
        .fold(request, |req, (key, value)| req.header(*key, *value))
}
