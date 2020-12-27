mod models;

use crate::models::{APP_TITLE, APP_VERSION};
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
    .version(APP_VERSION)
    .subcommand(
      SubCommand::with_name("download")
        .about("Download the Bing photo of the day and set is as desktop wallpaper or show it into a desktop notification")
        .arg(
          Arg::with_name("output-path")
            .long("output-path")
            .short("o")
            .takes_value(true)
            .value_name("OUTPUT_PATH")
            .help("Output path where save downloaded wallpaper (e.g. /home/user/wallpaper.jpg)"),
        )
        .arg(
          Arg::with_name("prev-days")
            .long("prev-days")
            .short("p")
            .takes_value(true)
            .value_name("PREV_DAYS")
            .default_value("0")
            .validator(|val: String| {
              // Try to parse
              match val.parse::<i8>() {
                Ok(val_int) => {
                  // Check constraints
                  if val_int >= 0 && val_int <= 14 {
                    Ok(())
                  } else {
                    Err(String::from("Value must be between 0 and 14"))
                  }
                },
                Err(_) => Err(String::from("Value must be an integer"))
              }
            })
            .help("Specify which image to download, i.e. download the image of the day of x days ago (max 14 days ago)."),
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
