use crate::models::BingDataPayload;
use bytes::Bytes;
use reqwest::header::ACCEPT;
use reqwest::Url;

const BING_BASE_URL: &'static str = "https://bing.com";
const BING_DATA_PATH: &'static str = "/HPImageArchive.aspx";

// const BING_URL: &'static str = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=8";

pub struct BingClient {}

impl BingClient {
  pub async fn fetch_bing_data() -> Result<BingDataPayload, reqwest::Error> {
    let mut url = Url::parse(BING_BASE_URL).unwrap();
    url.set_path(BING_DATA_PATH);
    url
      .query_pairs_mut()
      .append_pair("format", "js")
      .append_pair("idx", "0")
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
