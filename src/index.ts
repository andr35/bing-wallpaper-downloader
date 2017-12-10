import {Payload} from './payload';
import {WriteStream} from "fs";
import {exec, execSync} from 'child_process';
import {notify} from 'node-notifier';

import * as fs from 'fs';
import * as path from 'path';
import * as https from 'https';
// import * as Jimp from 'jimp';

const NG_URL = 'https://www.nationalgeographic.com/photography/photo-of-the-day/_jcr_content/.gallery.json';
const PHOTO_PATH = path.join(process.env['HOME'], '.wallpaper.jpg');

// Args
const SHOW_NOTIFICATION = process.argv.indexOf('show-notification') !== -1;
const FORCE_DOWNLOAD = process.argv.indexOf('force-download') !== -1;

if (isWallpaperAlreadySet() && !FORCE_DOWNLOAD) { // && !SHOW_NOTIFICATION
  console.log('> Wallpaper already downloaded');
} else {

  fetchData()
    .then(res => {
      const photoUrl = res.items[0].url + res.items[0].originalUrl;
      const file = fs.createWriteStream(PHOTO_PATH);
      fetchPhoto(photoUrl, file)
        .then(done => {

          const title = res.items[0].title;
          const caption = res.items[0].caption.replace(/<.?.>/g, '');


          if (SHOW_NOTIFICATION) {
            showNotification(title, caption);

          } else {

            // writeCaption(title, caption)
            // .then(done => {
            setWallpaper(PHOTO_PATH)
              .then(done => console.log('> Wallpaper set.'))
              .catch(err => exitWithError(err));
            // })
            // .catch(err => exitWithError(err));
          }

        })
        .catch(err => exitWithError(err));
    })
    .catch(err => exitWithError(err));
}

function fetchData(): Promise<Payload> {
  return new Promise<Payload>((resolve, reject) => {

    https.get(NG_URL, res => {
      const status = res.statusCode;
      res.statusCode
      const contentType = res.headers['content-type'];

      if (status !== 200) {
        res.resume();
        return reject(`Error while fetching data (status code "${status}").`);
      }

      if (!/^application\/json/.test(contentType as string)) {
        res.resume();
        return reject(`Error while fetching data (not a json content "${contentType}").`);
      }

      res.setEncoding('utf8');
      let rawData = '';
      res.on('data', chunk => rawData += chunk);
      res.on('end', () => resolve(JSON.parse(rawData)));

    }).on('error', err => reject(err));
  });
}

function fetchPhoto(url: string, stream: WriteStream): Promise<any> {
  return new Promise<boolean>((resolve, reject) => {
    https.get(url, res => {
      const status = (res as any).statusCode;

      if (status !== 200) {
        res.resume();
        return reject(`Error while fetching photo (status code "${status}").`);
      }
      res.pipe(stream);
      res.on('end', () => resolve(true));
    }).on('error', err => reject(err));
  });
}

function setWallpaper(photoPath: string): Promise<boolean> {
  return new Promise<boolean>((resolve, reject) => {
    const key = isCinnamon() ? 'org.cinnamon.desktop.background' : 'org.gnome.desktop.background';
    exec(`gsettings set ${key} picture-uri file://${photoPath}`, (error, stdout, stderr) => {
      return !error ? resolve(true) : reject(error)
    });
  });
}

// function writeCaption(title: string, caption: string): Promise<boolean> {
//   return new Promise<boolean>((resolve, reject) => {
//     let image: Jimp;
//     Jimp.read(PHOTO_PATH)
//       .then(i => image = i)
//       .then(() => (Jimp as any).loadFont(Jimp.FONT_SANS_16_BLACK))
//       .then(font => (image as any).print(font, 10, image.bitmap.height - 130, title).write(PHOTO_PATH))
//       .then(() => (Jimp as any).loadFont(Jimp.FONT_SANS_8_BLACK))
//       .then(font => (image as any).print(font, title.length * 11, image.bitmap.height - 122, caption).write(PHOTO_PATH))
//       .then(() => resolve(true))
//       .catch(err => reject(err));
//   });
// }

function isWallpaperAlreadySet(): boolean {
  const exists = fs.existsSync(PHOTO_PATH);
  if (exists) {
    const stats = fs.statSync(PHOTO_PATH);
    stats.birthtime.setHours(0);
    stats.birthtime.setMinutes(0);
    return (new Date().getTime() - stats.birthtime.getTime()) < (24 * 60 * 60 * 1000); // image is older than 24h
  }
  return false;
}

function isCinnamon(): boolean {
  try {
    return execSync('gsettings writable org.cinnamon.desktop.background picture-uri').toString('utf8').indexOf('true') !== -1;
  } catch (e) {
    return false;
  }
}

function showNotification(title: string = 'Unknown', message: string = 'National Geographic photo of the day') {
  notify({
    title: `Wallpaper - ${title}`,
    message,
    icon: PHOTO_PATH,
    urgency: 'critical'
  } as any);
}


function exitWithError(err: string) {
  console.error('> Error', err);
  process.exit(-1);
}
