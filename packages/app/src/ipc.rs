use std::collections::HashMap;

use axum::body::BoxBody;
use http::{HeaderName, HeaderValue, Request, Response, Uri};
use hyper::Body;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct IpcRequest {
    pub method: String,
    pub url: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

impl From<IpcRequest> for Request<Body> {
    fn from(request: IpcRequest) -> Self {
        let uri = Uri::builder()
            .scheme("http")
            .authority("localhost")
            .path_and_query(request.url)
            .build()
            .unwrap();

        let mut builder = Request::builder().method(&*request.method).uri(uri);

        builder
            .headers_mut()
            .unwrap()
            .extend(request.headers.into_iter().map(|(key, val)| {
                (
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(&val).unwrap(),
                )
            }));

        builder
            .body(request.body.unwrap_or_default().into())
            .unwrap()
    }
}

#[derive(Serialize)]
pub struct IpcResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl IpcResponse {
    pub async fn from_http_response(response: Response<BoxBody>) -> Result<Self, axum::Error> {
        let (parts, body) = response.into_parts();

        Ok(IpcResponse {
            status: parts.status.as_u16(),
            headers: parts
                .headers
                .iter()
                .map(|(key, val)| (key.to_string(), val.to_str().unwrap().to_owned()))
                .collect(),
            body: {
                let bytes = hyper::body::to_bytes(body).await?.into();

                String::from_utf8(bytes).unwrap()
            },
        })
    }
}
