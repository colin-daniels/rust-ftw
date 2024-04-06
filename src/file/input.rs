use http::{HeaderName, HeaderValue, Request, Uri};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HttpVersion {
    #[serde(rename = "HTTP/1.0")]
    Http1_0,
    #[serde(rename = "HTTP/1.1")]
    #[default]
    Http1_1,
    #[serde(rename = "HTTP/2", alias = "HTTP/2.0")]
    Http2_0,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "InputData")]
#[serde(into = "Option<String>")]
pub struct Data(Option<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    /// <IP or DNS> def = localhost
    #[serde(default = "default::addr")]
    pub dest_addr: String,
    #[serde(default = "default::port")]
    pub port: u32,
    #[serde(default = "default::method")]
    pub method: String,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    /// http or https
    #[serde(default = "default::protocol")]
    pub protocol: String,
    #[serde(default = "default::uri")]
    pub uri: String,
    #[serde(default = "Default::default")]
    pub version: HttpVersion,
    #[serde(default)]
    pub data: Data,
    /// If there are multiple stages and save cookie is set, it will automatically be provided in
    /// the next stage if the site in question provides the Set-Cookie response header.
    /// Default = false.
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub save_cookie: bool,
    /// The framework will take care of certain things automatically like setting content-length,
    /// encoding, etc. When stop_magic is on, the framework will not do anything automagically.
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub stop_magic: bool,
    /// Description: This argument will take a base64 encoded string that will be decoded and sent
    /// through as the request. It will override all other settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoded_request: Option<String>,
    /// Description: This argument will take a unencoded string that will be sent through as the
    /// request. It will override all other settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_request: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum InputData {
    None,
    // Note: literals \r and \n will be replaced be replaced with CRLF when stop_magic is on. Note: if
    // urlencoded content-type header is provided and parameters aren't in name=value form, data will
    // be made empty, unless stop_magic is on.
    Text(String),
    Lines(Vec<String>),
}

impl From<InputData> for Data {
    fn from(data: InputData) -> Self {
        // convert request lines into a single string
        Self(match data {
            InputData::None => None,
            InputData::Text(text) => Some(text),
            InputData::Lines(lines) => Some(lines.join("\r\n")),
        })
    }
}

impl From<Data> for Option<String> {
    fn from(data: Data) -> Self {
        data.0
    }
}

mod default {
    pub fn addr() -> String {
        "localhost".into()
    }
    pub fn port() -> u32 {
        80
    }
    pub fn method() -> String {
        "GET".into()
    }
    pub fn protocol() -> String {
        "http".into()
    }
    pub fn uri() -> String {
        "/".into()
    }
}

impl Data {
    pub(super) fn replace_escaped_crlf(&mut self) {
        lazy_static::lazy_static! {
            static ref ESCAPED_CRLF: Regex = Regex::new(r"\\r\\n").unwrap();
        }

        self.0 = self
            .0
            .as_ref()
            .map(|text| ESCAPED_CRLF.replace_all(text.as_str(), "\r\n").into());
    }
}

impl Input {
    pub fn do_magic(&mut self) {
        // FTW does some things automatically if stop_magic == false, so we do them here.
        if !self.stop_magic {
            // replace "\\r\\n" with actual CRLFs
            self.data.replace_escaped_crlf();

            // default content type to "application/x-www-form-urlencoded" if we have any data
            if let Some(text) = &self.data.0 {
                if !text.is_empty()
                    && !self.headers.contains_key("Content-Type")
                    && !self.headers.contains_key("content-type")
                {
                    self.headers.insert(
                        "Content-Type".into(),
                        "application/x-www-form-urlencoded".into(),
                    );
                }
            }
        }
    }

    pub fn uri(&self) -> Result<Uri, http::Error> {
        Uri::builder()
            .scheme(self.protocol.as_str())
            .authority(format!("localhost:{}", self.port))
            .path_and_query(self.uri.as_str())
            .build()
    }

    pub fn request(&self) -> Result<Request<Vec<u8>>, http::Error> {
        let mut builder = Request::builder()
            .method(self.method.as_str())
            .uri(self.uri()?);

        for (header, value) in &self.headers {
            let name = HeaderName::from_bytes(header.as_bytes())?;
            let value: HeaderValue = value.parse()?;
            builder = builder.header(name, value);
        }

        if let Some(body) = self.data.0.as_ref() {
            builder.body(body.clone().into_bytes())
        } else {
            builder.body(Default::default())
        }
    }
}

/// Utility to replace invalid URI characters. TBD whether this
/// should actually be done or if we should add a separate way of
/// sending "invalid" requests.
#[allow(dead_code)]
fn replace_invalid_query_string_chars(uri: &str) -> String {
    // replace any unescaped characters in the query string
    let mut uri_new = String::with_capacity(uri.len() * 3);
    let mut past_query_string = false;

    for &b in uri.as_bytes().iter() {
        if !past_query_string {
            past_query_string = b == b'?';
            uri_new.push(char::from(b));
        } else {
            match b {
                b' ' => uri_new.push_str("%20"),
                b'"' => uri_new.push_str("%22"),
                b'<' => uri_new.push_str("%3C"),
                b'>' => uri_new.push_str("%3E"),
                b'?' => uri_new.push_str("%3F"),
                _ => uri_new.push(char::from(b)),
            }
        }
    }
    uri_new
}
