import {Notification} from 'node-notifier';
import * as notifier from 'node-notifier';

class NotificationService {

  showNotification(iconPath: string, title: string = 'Unknown', message: string = 'National Geographic photo of the day') {
    notifier.notify({
      title: `Wallpaper - ${title}`,
      message,
      icon: iconPath,
      contentImage: iconPath,
      urgency: 'critical'
    } as Notification);
  }

}

export default new NotificationService();
