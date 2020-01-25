import * as program from 'commander';
import {CommanderStatic} from 'commander';
import {createWriteStream} from 'fs';
import {AppConstants} from '../models/app-constants';
import downloaderService from './downloader.service';
import notificationService from './notification.service';
import wallpaperService from './wallpaper.service';


export function runProgram() {

  program
    .version('3.0.0')
    .option('-f, --force', 'Force download')
    .option('-n, --notification', 'Show a notification instead of apply wallpaper')
    .option('-g, --national-geographic', 'Use National Geographic as source of wallpapers (default)')
    .option('-b, --bing', 'Use Bing as source of wallpapers');


  program
    .command('now')
    .description('Download and apply wallpaper')
    .action(async () => {
      await runWallpaperScript(0, program.force, program.notification, getSources(program));
    });

  program
    .command('prev-day <day>')
    .description('Download and apply wallpaper of a previous day')
    .action(async dayString => {

      const day = parseInt(dayString);

      if (Number.isNaN(day)) {
        throw new Error('Invalid argument');
      }

      await runWallpaperScript(day, program.force, program.notification, getSources(program));
    });


  program.parse(process.argv)
}


async function runWallpaperScript(prevDay: number = 0, force: boolean, showNotification: boolean, sources: string[]) {


  if (wallpaperService.isWallpaperAlreadySet(AppConstants.DEFAULT_PHOTO_PATH) && !force) { // && !SHOW_NOTIFICATION
    console.log('> Wallpaper already downloaded');
  } else {

    console.log(prevDay ? `Downloading photo of ${prevDay} days backward...` : 'Downloading photo of today');

    try {

      const photoData = await getPhotoData(prevDay, sources);

      // Checkif requested photo of the day exists in NG gallery (NB: only photos of the current month are available)
      if (!photoData) {
        console.log('There is no photo for the day requested');
        process.exit(-1);
      }

      for (let i = 0; i < photoData.length; i++) {
        const currentPhotoData = photoData[i];

        let filePath = showNotification ? AppConstants.DEFAULT_PHOTO_PATH_TMP : AppConstants.DEFAULT_PHOTO_PATH;
        filePath = `${filePath}_${i}`;

        const writer = createWriteStream(filePath);
        await downloaderService.fetchPhoto(currentPhotoData.url, writer);

        if (showNotification) {
          notificationService.showNotification(filePath, currentPhotoData.title, currentPhotoData.caption);
        } else {

          if (i === 0) { // Only the first image
            // writeCaption(title, caption)
            // .then(done => {
            await wallpaperService.setWallpaper(filePath);
            console.log('> Wallpaper set.');
            // })
            // .catch(err => exitWithError(err));
          }
        }

      }


    } catch (err) {
      console.log('Error!', err);
      process.exit(-1);
    }

  }

}

async function getPhotoData(prevDay: number = 0, sources: string[]): Promise<{url: string, title: string, caption: string}[] | null> {

  const photoData: {url: string, title: string, caption: string}[] = [];

  if (sources.includes('ng')) {

    const ngRes = await downloaderService.fetchNGData();
    const ngItem = ngRes.items[prevDay];

    // Checkif requested photo of the day exists in NG gallery (NB: only photos of the current month are available)
    if (ngItem) {
      // const photoUrl = item.url + item.originalUrl;
      const photoUrl = ngItem.originalUrl || ngItem.url || ngItem.image.uri;
      photoData.push({url: photoUrl, title: ngItem.image.title, caption: ngItem.image.caption.replace(/<.?.>/g, '')});
    }
  }

  if (sources.includes('bing')) {

    const bingRes = await downloaderService.fetchBingData(prevDay);
    const bingItem = bingRes.images[prevDay % 8];

    // Checkif requested photo of the day exists in NG gallery (NB: only photos of the current month are available)
    if (bingItem) {
      // const photoUrl = item.url + item.originalUrl;
      const photoUrl = `https://www.bing.com${bingItem.urlbase}_1920x1080.jpg`;
      photoData.push({url: photoUrl, title: bingItem.title, caption: bingItem.copyright});
    }

  }

  return photoData.length > 0 ? photoData : null;
}


function getSources(program: CommanderStatic): string[] {
  const sources = [program.nationalGeographic && 'ng', program.bing && 'bing'].filter(s => s !== undefined);
  return sources.length > 0 ? sources : ['ng'];
}
