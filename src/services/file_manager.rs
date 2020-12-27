use crate::models::AppError;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

const DEFAULT_IMG_NAME: &str = ".wallpaper.jpg";

pub struct FileManager {}

impl FileManager {
  pub fn save_wallpaper(data: &[u8], output_path: Option<&str>) -> Result<String, Box<dyn Error>> {
    let path = match output_path {
      Some(p) => PathBuf::from(p),
      None => std::env::temp_dir().join(DEFAULT_IMG_NAME),
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

  pub fn copy_into_default_wallpaper_location(curr_path: &str) -> Result<String, Box<dyn Error>> {
    // Try to copy wallpaper in picture dirs -> if fail, do nothing
    let output_path_buf = match dirs::picture_dir() {
      Some(picture_dir) => picture_dir.join(DEFAULT_IMG_NAME),
      None => {
        log::warn!("Picture folder location is unknown. Wallpaper will not be copied.");
        PathBuf::from(curr_path)
      }
    };
    let output_path = output_path_buf.as_path();

    std::fs::copy(curr_path, output_path)?;

    return match output_path.to_str() {
      Some(st) => Ok(String::from(st)),
      None => Err(Box::new(AppError::new("Unknown wallpaper path"))),
    };
  }
}
