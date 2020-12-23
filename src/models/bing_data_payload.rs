use serde::Deserialize;

#[derive(Deserialize)] // Debug
pub struct BingDataPayload {
  pub images: Vec<BingDataPayloadImage>,
  pub tooltips: BingDataPayloadTooltips,
}

#[derive(Deserialize)] // Debug
pub struct BingDataPayloadImage {
  pub startdate: String,
  pub fullstartdate: String,
  pub enddate: String,
  pub url: String,
  pub urlbase: String,
  pub copyright: String,
  pub copyrightlink: String,
  pub title: String,
  pub quiz: String,
  pub wp: bool,
  pub hsh: String,
  pub drk: i8,
  pub top: i8,
  pub bot: i8,
  pub hs: Vec<i8>,
}

#[derive(Deserialize)] // Debug
pub struct BingDataPayloadTooltips {
  pub loading: String,
  pub previous: String,
  pub next: String,
  pub walle: String,
  pub walls: String,
}
