#[allow(unused)]
use serde::{de::DeserializeOwned, Serialize};
use web_sys::FormData;

use crate::error::{AuthorizeErrors, Error};

const API_ROOT_DEV: &str = env!("API_ROOT_DEV");
const API_ROOT: &str = env!("API_ROOT");

pub async fn request<B, T>(method: reqwasm::http::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = matches!(
        method,
        reqwasm::http::Method::POST | reqwasm::http::Method::PUT
    );

    let url = format!("{}{}", API_ROOT, url);

    let mut req = reqwasm::http::Request::new(&url)
        .method(method)
        .header("Content-Type", "application/json");

    if allow_body {
        let body_json =
            serde_json::to_string(&body).map_err(|ex| Error::SerializeError(ex.to_string()))?;
        req = req.body(body_json);
    }

    let response = req
        .send()
        .await
        .map_err(|ex| Error::HttpRequestError(ex.to_string()))?;

    handle_response(response).await
}

pub async fn request_form<T>(
    method: reqwasm::http::Method,
    url: String,
    form_data: FormData,
) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT_DEV, url);

    let req = reqwasm::http::Request::new(&url)
        .method(method)
        .body(form_data);

    let response = req
        .send()
        .await
        .map_err(|ex| Error::HttpRequestError(ex.to_string()))?;

    handle_response(response).await
}

async fn handle_response<T>(response: reqwasm::http::Response) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    if response.ok() {
        let data: T = response
            .json()
            .await
            .map_err(|ex| Error::DeserializeError(ex.to_string()))?;
        Ok(data)
    } else {
        let status = response.status();
        let error_txt = response.text().await.unwrap_or_default();

        match status {
            400 => Err(Error::BadRequest),
            401 => {
                let data: Result<AuthorizeErrors, _> = serde_json::from_str(&error_txt);
                if let Ok(data) = data {
                    Err(Error::Unauthorized(data.message))
                } else {
                    Err(Error::DeserializeError(
                        "Unauthorized Error 401".to_string(),
                    ))
                }
            }
            403 => Err(Error::Forbidden),
            404 => {
                let data: Result<AuthorizeErrors, _> = serde_json::from_str(&error_txt);
                Err(Error::NotFound(
                    data.map(|d| d.message).unwrap_or_else(|_| error_txt),
                ))
            }
            500 => {
                let data: Result<AuthorizeErrors, _> = serde_json::from_str(&error_txt);
                Err(Error::InternalServerError(
                    data.map(|d| d.message).unwrap_or_else(|_| error_txt),
                ))
            }
            _ => Err(Error::HttpRequestError(format!(
                "Unexpected status code: {}",
                status
            ))),
        }
    }
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwasm::http::Method::POST, url, body).await
}

pub async fn request_form_post<T>(url: String, form_data: FormData) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request_form(reqwasm::http::Method::POST, url, form_data).await
}
