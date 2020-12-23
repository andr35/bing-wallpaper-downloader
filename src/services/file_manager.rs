use crate::models::AppError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const IMG_NAME: &str = ".wallpaper.jpg";

pub struct FileManager {}

impl FileManager {
  pub fn save_wallpaper(data: &[u8], output_path: Option<&str>) -> Result<String, Box<dyn Error>> {
    // let path = std::path::Path::new("/workspaces/national-geographic-wallpaper").join(IMG_NAME);
    let path = match output_path {
      Some(p) => PathBuf::from(p),
      None => dirs::home_dir().unwrap().join(IMG_NAME),
    };
    let path = path.as_path();

    let mut file = match File::create(path) {
      Ok(file) => file,
      Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
    };

    file.write_all(data)?;

    return match path.to_str() {
      Some(st) => Ok(String::from(st)),
      None => Err(Box::new(AppError::new("Unknown img path"))),
    };
  }
}
