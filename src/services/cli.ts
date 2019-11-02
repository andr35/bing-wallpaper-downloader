import * as program from 'commander';
import {createWriteStream} from 'fs';
import {AppConstants} from '../models/app-constants';
import downloaderService from './downloader.service';
import notificationService from './notification.service';
import wallpaperService from './wallpaper.service';


export function runProgram() {

  program
    .version('2.0.0')
    .option('-f, --force', 'Force download')
    .option('-n, --notification', 'Show a notification instead of apply wallpaper');


  program
    .command('now')
    .description('Download and apply wallpaper')
    .action(async () => {
      await runWallpaperScript(0, program.force, program.notification);
    });

  program
    .command('prev-day <day>')
    .description('Download and apply wallpaper of a previous day')
    .action(async dayString => {

      const day = parseInt(dayString);

      if (Number.isNaN(day)) {
        throw new Error('Invalid argument');
      }

      await runWallpaperScript(day, program.force, program.notification);
    });


  program.parse(process.argv)
}


async function runWallpaperScript(prevDay: number = 0, force: boolean, showNotification: boolean) {


  if (wallpaperService.isWallpaperAlreadySet(AppConstants.DEFAULT_PHOTO_PATH) && !force) { // && !SHOW_NOTIFICATION
    console.log('> Wallpaper already downloaded');
  } else {

    console.log(prevDay ? `Downloading photo of ${prevDay} days backward...` : 'Downloading photo of today');

    try {
      const res = await downloaderService.fetchData();

      const item = res.items[prevDay];
      // Checkif requested photo of the day exists in NG gallery (NB: only photos of the current month are available)
      if (!item) {
        console.log('There is no photo for the day requested');
        process.exit(-1);
      }


      // const photoUrl = item.url + item.originalUrl;
      const photoUrl = item.originalUrl || item.url || item.image.uri;

      const filePath = showNotification ? AppConstants.DEFAULT_PHOTO_PATH_TMP : AppConstants.DEFAULT_PHOTO_PATH;
      const writer = createWriteStream(filePath);
      await downloaderService.fetchPhoto(photoUrl, writer);

      const title = item.image.title;
      const caption = item.image.caption.replace(/<.?.>/g, '');

      if (showNotification) {
        notificationService.showNotification(filePath, title, caption);

      } else {

        // writeCaption(title, caption)
        // .then(done => {
        await wallpaperService.setWallpaper(filePath);
        console.log('> Wallpaper set.');
        // })
        // .catch(err => exitWithError(err));
      }

    } catch (err) {
      console.log('Error!', err);
      process.exit(-1);
    }

  }

}
