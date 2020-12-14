use std::error::Error;

use clap::ArgMatches;

pub fn run(app_matches: ArgMatches) -> Result<(), Box<dyn Error>> {
  match app_matches.subcommand() {
    // Test cmd
    ("test", Some(sub_m)) => {
      let d = sub_m.value_of("debug");

      match d {
        Some(dd) => {
          println!("With debug val {}", dd);
        }
        None => {
          println!("With no debug");
        }
      }
    }

    _ => {
      println!("Ohoh... unknown cmd")
    }
  }

  Ok(())
}
