mod request;
mod response;

use crate::BASE_URL;
pub use request::*;
use reqwest::Client;
pub use response::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use slint::{Rgba8Pixel, SharedPixelBuffer};

pub async fn gen_url(url: &str) -> anyhow::Result<String> {
    let url = if url.starts_with("/") {
        format!("{}{}", BASE_URL, url)
    } else if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{}/{}", BASE_URL, url)
    };
    Ok(url)
}

pub async fn url_get<T>(url: &str) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let url = gen_url(url).await?;
    let response = reqwest::get(&url).await?;
    Ok(response.json::<T>().await?)
}

pub async fn url_get_bytes(url: &str) -> anyhow::Result<bytes::Bytes> {
    let url = gen_url(url).await?;
    let response = reqwest::get(&url).await?;
    Ok(response.bytes().await?)
}

pub async fn url_get_image(url: &str) -> anyhow::Result<image::DynamicImage> {
    let bytes = url_get_bytes(url).await?;
    let img = image::load_from_memory(&bytes)?;
    Ok(img)
}

pub async fn url_get_image_buffer(url: &str) -> anyhow::Result<SharedPixelBuffer<Rgba8Pixel>> {
    let img = url_get_image(url).await?;
    let img = img.into_rgba8();
    let img = SharedPixelBuffer::clone_from_slice(&img, img.width(), img.height());
    Ok(img)
}

pub async fn url_post<T, U>(url: &str, body: U) -> anyhow::Result<T>
where
    T: DeserializeOwned,
    U: Serialize,
{
    let url = gen_url(url).await?;
    let response = Client::new().post(&url).json(&body).send().await?;

    match response.status().is_success() {
        true => Ok(response.json::<T>().await?),
        false => {
            let text = response.text().await?;
            Err(anyhow::anyhow!("{}", text.trim()))
        }
    }
}
