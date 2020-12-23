use wallpaper;

pub struct WallpaperManager {}

impl WallpaperManager {
  pub fn set_as_wallpaper(img_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    match wallpaper::set_from_path(img_url) {
      Err(e) => panic!("Fail to set desktop wallpaper {}", e),
      _ => Ok(()),
    }
  }
}
