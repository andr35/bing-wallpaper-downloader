use crate::models::{BingDataPayload, BingDataPayloadImage};
use bytes::Bytes;
use reqwest::header::ACCEPT;
use reqwest::Url;
use std::convert::TryInto;

const BING_BASE_URL: &str = "https://bing.com";
const BING_DATA_PATH: &str = "/HPImageArchive.aspx";

// const BING_URL: &'static str = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8";

pub struct BingClient {}

impl BingClient {
  pub async fn fetch_bing_data_image(
    prev_days: i8,
  ) -> Result<BingDataPayloadImage, reqwest::Error> {
    let data = BingClient::fetch_bing_data(prev_days).await?;

    let img_in_array: usize = (prev_days - 7).try_into().unwrap_or_else(|_| 0);
    let img = match data.images.into_iter().nth(img_in_array) {
      Some(img) => img,
      None => panic!("Image not found"),
    };
    Ok(img)
  }

  async fn fetch_bing_data(idx: i8) -> Result<BingDataPayload, reqwest::Error> {
    let mut url = Url::parse(BING_BASE_URL).unwrap();
    url.set_path(BING_DATA_PATH);
    url
      .query_pairs_mut()
      .append_pair("format", "js")
      .append_pair("idx", &idx.to_string())
      .append_pair("n", "8")
      .finish();

    return Ok(
      reqwest::Client::new()
        .get(url.as_str())
        .header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<BingDataPayload>()
        .await?,
    );
  }

  pub async fn fetch_bing_photo(photo_path: &str) -> Result<Bytes, reqwest::Error> {
    let url = Url::parse(BING_BASE_URL).unwrap().join(photo_path).unwrap();

    return Ok(
      reqwest::Client::new()
        .get(url.as_str())
        .send()
        .await?
        .bytes()
        .await?,
    );
  }
}
