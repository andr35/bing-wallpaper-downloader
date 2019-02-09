import {homedir, tmpdir} from 'os';
import {join} from 'path';

export class AppConstants {
  static NG_URL = 'https://www.nationalgeographic.com/photography/photo-of-the-day/_jcr_content/.gallery.json?user.testname=none';

  static DEFAULT_PHOTO_PATH = join(homedir(), '.wallpaper.jpg');
  static DEFAULT_PHOTO_PATH_TMP = join(tmpdir(), 'wallpaper.jpg');


  private constructor() {}
}
