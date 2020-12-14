use clap::{App, Arg, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let matches = App::new("fake")
        .version("v0.1.0")
        .subcommand(
            SubCommand::with_name("test").about("Do something").arg(
                Arg::with_name("debug")
                    .short("d")
                    .takes_value(true)
                    .help("do some debug"),
            ),
        )
        .get_matches();

    return bing_wallpaper_downloader::run(matches);
}
