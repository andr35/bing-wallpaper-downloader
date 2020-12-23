use crate::models::APP_NAME;
use crate::services::WallpaperManager;
use notify_rust::{Notification, Timeout};

pub struct NotificationManager {}

impl NotificationManager {
  pub fn show_notification(
    title: &str,
    caption: &str,
    img_url: &str,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let summary = match title {
      "" => "Bing Wallpaper",
      t => t,
    };

    Notification::new()
      .summary(summary)
      .body(caption)
      .image_path(img_url)
      .appname(APP_NAME)
      .timeout(Timeout::Never)
      .action("default", "default")
      .action("set_wp", "Set as Wallpaper")
      .action("cancel", "Cancel")
      .show()?
      .wait_for_action(|action| match action {
        "set_wp" => {
          WallpaperManager::set_as_wallpaper(img_url).expect("Fail to set desktop wallpaper");
        }
        _ => (),
      });

    Ok(())
  }
}
