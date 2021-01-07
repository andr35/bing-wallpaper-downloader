//! Bing Wallpaper Downloader
//!
//! Simple CLI application written in [Rust](https://www.rust-lang.org/) to download
//! the Bing Image of the day and, optionally, set it as wallpaper or show a desktop
//! notification with the image and its caption.

pub mod models;
pub mod services;

use crate::models::AppError;
use crate::services::{BingClient, FileManager, NotificationManager, WallpaperManager};
use clap::ArgMatches;
use std::error::Error;

/// Given the commands and arguments passed from the user, it executes
/// the commands.
pub async fn run(app_matches: ArgMatches<'_>) -> Result<(), Box<dyn Error>> {
  // Run commands
  match app_matches.subcommand() {
    // Download command
    ("download", Some(sub_m)) => {
      log::info!("Download image...");

      let prev_days = sub_m
        .value_of("prev-days")
        .expect("Missing prev day")
        .parse::<i8>()?;

      // First -> Get image info
      let img = BingClient::fetch_bing_data_image(prev_days).await?;
      let img_url = img.url.clone();
      let img_bytes = BingClient::fetch_bing_photo(&img_url).await?;
      // Save image in fs
      let img_save_path =
        FileManager::save_wallpaper(img_bytes.as_ref(), sub_m.value_of("output-path"))?;

      log::info!("Image downloaded in folder {}", &img_save_path);

      // Show notification
      if sub_m.is_present("show-notification") {
        log::info!("Show notification with photo of the day...");
        NotificationManager::show_notification("", &img.copyright, &img_save_path)?;
      }

      // Set as wallpaper
      if sub_m.is_present("set-as-wallpaper") {
        WallpaperManager::set_as_wallpaper(&img_save_path)?;
        log::info!("Photo of the day set a desktop wallpaper!");
      }

      Ok(())
    }

    _ => {
      log::error!("Ohoh... unknown command. Use --help for more information");
      Err(Box::new(AppError::new("Unknown command")))
    }
  }
}
