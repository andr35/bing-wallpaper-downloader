use crate::services::FileManager;

pub struct WallpaperManager {}

impl WallpaperManager {
  pub fn set_as_wallpaper(img_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wallpaper_path = FileManager::copy_into_default_wallpaper_location(&img_url)?;

    match wallpaper::set_from_path(&wallpaper_path) {
      Err(e) => panic!("Fail to set desktop wallpaper {}", e),
      _ => Ok(()),
    }
  }
}
