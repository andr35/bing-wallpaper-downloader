use crate::models::APP_NAME;
use crate::services::WallpaperManager;
use notify_rust::{Notification, Timeout};

/// Utils service to create notifications
pub struct NotificationManager {}

impl NotificationManager {
  /// Creates and show a desktop notification containing the title, the caption and the provided image.
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
      .timeout(Timeout::Milliseconds(60000)) // Expire after 60s
      .action("default", "default")
      .action("set_wp", "Set as Wallpaper")
      .action("cancel", "Cancel")
      .show()?
      .wait_for_action(|action| match action {
        "set_wp" => {
          WallpaperManager::set_as_wallpaper(img_url).expect("Fail to set desktop wallpaper");
        }
        _ => {
          log::info!("Action cancelled");
        }
      });

    Ok(())
  }
}
