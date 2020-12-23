mod models;

use crate::models::APP_TITLE;
use clap::{App, Arg, SubCommand};
use log::LevelFilter;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Init logger
  env_logger::builder()
    .filter_level(LevelFilter::Info) // TODO change
    .format_timestamp(None)
    .format_module_path(false)
    .init();

  // Parse CLI args
  let matches = App::new(APP_TITLE)
    .version("v0.1.0")
    .subcommand(
      SubCommand::with_name("download")
        .about("Download the Bing photo of the day")
        .arg(
          Arg::with_name("output-path")
            .long("output-path")
            .short("o")
            .takes_value(true)
            .help("Output path where save downloaded wallpaper (e.g. /home/user/wallpaper.jpg)"),
        )
        .arg(
          Arg::with_name("show-notification")
            .long("show-notification")
            .short("n")
            .help("Show a system notification with the wallpaper's caption"),
        )
        .arg(
          Arg::with_name("set-as-wallpaper")
            .long("set-as-wallpaper")
            .short("s")
            .help("Set the downloaded image a s desktop wallpaper"),
        ),
    )
    .get_matches();

  // Run app
  let res = bing_wallpaper_downloader::run(matches).await;

  match res {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("Application error: {}", e);
      // eprintln!("Application error: {}", e);
      process::exit(1);
    }
  }
}
