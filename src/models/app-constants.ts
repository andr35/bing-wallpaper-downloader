import {homedir, tmpdir} from 'os';
import {join} from 'path';

export class AppConstants {
  static NG_URL = 'https://www.nationalgeographic.com/photography/photo-of-the-day/_jcr_content/.gallery.json';
  static BING_URL = 'https://www.bing.com/HPImageArchive.aspx?format=js&idx=TMPL_IDX&n=TMPL_NUM'; // mkt=en-US

  static DEFAULT_PHOTO_PATH = join(homedir(), '.wallpaper.jpg');
  static DEFAULT_PHOTO_PATH_TMP = join(tmpdir(), 'wallpaper.jpg');


  private constructor() {}
}
