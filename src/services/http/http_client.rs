use crate::error::{Error, Result};

use reqwest::{header::HeaderValue, Client, Response};
use url::Url;

#[derive(Debug, Clone)]
pub struct HeaderMap {
    headers: reqwest::header::HeaderMap,
}

impl HeaderMap {
    // 创建一个新的 HeaderMap
    pub fn new() -> Self {
        HeaderMap {
            headers: reqwest::header::HeaderMap::new(),
        }
    }

    pub fn insert(&mut self, key: &'static str, value: String) -> Result<()> {
        let header_value = HeaderValue::from_str(&value)
            .map_err(|_| Error::ErrorMessage("invalid headerValue".into()))?;
        self.headers.insert(key, header_value);
        Ok(())
    }

    // 获取 header 的值
    pub fn get(&self, key: &'static str) -> Option<String> {
        self.headers
            .get(key)
            .map(|v| v.to_str().unwrap_or_default().to_string())
    }

    // 返回内部的 HeaderMap
    pub fn inner(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }
}

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    base_url: Option<Url>,
    default_headers: HeaderMap,
}

impl HttpClient {
    // 创建新的 HTTP 客户端
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
            base_url: None,
            default_headers: HeaderMap::new(),
        }
    }

    // 设置 base_url
    pub fn set_base_url(&mut self, base_url: &str) -> Result<()> {
        let url = Url::parse(base_url)?;
        self.base_url = Some(url);
        Ok(())
    }

    // 设置默认的请求头
    pub fn set_default_headers(
        &mut self,
        headers: Vec<(&'static str, String)>, // 直接使用 Vec<(&'static str, String)>
    ) -> Result<()> {
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            header_map.insert(key, value)?;
        }
        self.default_headers = header_map;
        Ok(())
    }

    // 发送 GET 请求
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

    // 发送 POST 请求
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

    // 发送 PUT 请求
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

    // 发送 DELETE 请求
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

    // 构建完整 URL
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

pub fn parse_url(url: &str, query: Option<Vec<(String, String)>>) -> Result<Url> {
    let mut url = Url::parse(url)?;
    if let Some(query_params) = query {
        let query_pairs: Vec<(String, String)> = query_params.into_iter().collect();
        url.query_pairs_mut().extend_pairs(query_pairs);
    }
    Ok(url)
}
