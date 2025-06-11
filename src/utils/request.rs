use std::pin::Pin;

use bytes::Bytes;
use futures_util::{Stream, StreamExt};
use reqwest::{Client, Response, header::HeaderValue};
use url::Url;

use crate::error::{Error, Result};

pub type ByteStream = Pin<Box<dyn Stream<Item = crate::error::Result<Bytes>> + Send>>;

/// Wrapper for HTTP headers used in request construction.
#[derive(Debug, Clone)]
pub struct HeaderMap {
    headers: reqwest::header::HeaderMap,
}

impl HeaderMap {
    /// Create a new empty HeaderMap.
    pub fn new() -> Self {
        HeaderMap {
            headers: reqwest::header::HeaderMap::new(),
        }
    }

    /// Insert a header key-value pair.
    pub fn insert(&mut self, key: &'static str, value: String) -> Result<()> {
        let header_value = HeaderValue::from_str(&value)
            .map_err(|_| Error::ErrorMessage("invalid headerValue".into()))?;
        self.headers.insert(key, header_value);
        Ok(())
    }

    /// Get the value of a header as String.
    pub fn get(&self, key: &'static str) -> Option<String> {
        self.headers
            .get(key)
            .map(|v| v.to_str().unwrap_or_default().to_string())
    }

    /// Get reference to the internal reqwest HeaderMap.
    pub fn inner(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }
}

/// An HTTP request builder and executor with base URL and default headers.
#[derive(Debug)]
pub struct Request {
    client: Client,
    base_url: Option<Url>,
    default_headers: HeaderMap,
}

impl Request {
    /// Create a new Request client.
    pub fn new() -> Self {
        Request {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
            base_url: None,
            default_headers: HeaderMap::new(),
        }
    }

    pub fn with_timeout(timeout_sec: u64) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_sec))
            .build()
            .map_err(|e| Error::ErrorMessage(e.to_string()))?;
        Ok(Request {
            client,
            base_url: None,
            default_headers: HeaderMap::new(),
        })
    }

    /// Set the base URL for all requests.
    pub fn set_base_url(&mut self, base_url: &str) -> Result<()> {
        let url = Url::parse(base_url)?;
        self.base_url = Some(url);
        Ok(())
    }

    /// Set default headers to be applied on all requests.
    pub fn set_default_headers(&mut self, headers: Vec<(&'static str, String)>) -> Result<()> {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(key, value)?;
        }
        self.default_headers = header_map;
        Ok(())
    }

    /// Send a GET request.
    pub async fn get(
        &self,
        endpoint: &str,
        query: Option<Vec<(String, String)>>,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, query)?;
        let mut request = self.client.get(url.as_str());
        let combined_headers = self.merge_headers(headers)?;
        request = request.headers(combined_headers.inner().clone());
        let response = request.send().await?;
        Ok(response)
    }

    /// Send a POST request with JSON body.
    pub async fn post(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.post(url).json(body);
        let combined_headers = self.merge_headers(headers)?;
        request = request.headers(combined_headers.inner().clone());
        let response = request.send().await?;
        Ok(response)
    }

    /// Send a PUT request with JSON body.
    pub async fn put(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.put(url).json(body);
        let combined_headers = self.merge_headers(headers)?;
        request = request.headers(combined_headers.inner().clone());
        let response = request.send().await?;
        Ok(response)
    }

    /// Send a DELETE request.
    pub async fn delete(
        &self,
        endpoint: &str,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.delete(url);
        let combined_headers = self.merge_headers(headers)?;
        request = request.headers(combined_headers.inner().clone());
        let response = request.send().await?;
        Ok(response)
    }

    /// Send a streaming POST request and return the response stream.
    pub async fn post_stream(
        &self,
        endpoint: &str,
        body: &serde_json::Value,
        headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<ByteStream> {
        let url = self.build_url(endpoint, None)?;
        let mut request = self.client.post(url).json(body);
        let combined_headers = self.merge_headers(headers)?;
        request = request.headers(combined_headers.inner().clone());

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(Error::ErrorMessage(format!(
                "Unexpected status: {}",
                response.status()
            )));
        }

        let stream = response
            .bytes_stream()
            .map(|chunk_result| chunk_result.map_err(Error::from));
        Ok(Box::pin(stream))
    }

    /// Build a full URL by combining base URL, endpoint, and optional query parameters.
    fn build_url(&self, endpoint: &str, query: Option<Vec<(String, String)>>) -> Result<Url> {
        let mut url = if let Some(base_url) = &self.base_url {
            base_url.join(endpoint)?
        } else {
            Url::parse(endpoint)?
        };

        if let Some(query_params) = query {
            let query_pairs: Vec<(String, String)> = query_params.into_iter().collect();
            url.query_pairs_mut().extend_pairs(query_pairs);
        }

        Ok(url)
    }

    /// Merge default headers with custom request headers.
    fn merge_headers(
        &self,
        custom_headers: Option<Vec<(&'static str, String)>>,
    ) -> Result<HeaderMap> {
        let mut combined_headers = self.default_headers.clone();
        if let Some(header_vec) = custom_headers {
            for (key, value) in header_vec {
                combined_headers.insert(key, value)?;
            }
        }
        Ok(combined_headers)
    }
}

/// Parse a full URL with optional query parameters.
pub fn parse_url(url: &str, query: Option<Vec<(String, String)>>) -> Result<Url> {
    let mut url = Url::parse(url)?;
    if let Some(query_params) = query {
        let query_pairs: Vec<(String, String)> = query_params.into_iter().collect();
        url.query_pairs_mut().extend_pairs(query_pairs);
    }
    Ok(url)
}
