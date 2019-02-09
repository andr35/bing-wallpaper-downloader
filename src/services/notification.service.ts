import {Notification, notify} from 'node-notifier';

class NotificationService {

  showNotification(iconPath: string, title: string = 'Unknown', message: string = 'National Geographic photo of the day') {
    notify({
      title: `Wallpaper - ${title}`,
      message,
      icon: iconPath,
      urgency: 'critical'
    } as Notification);
  }

}

export default new NotificationService();
