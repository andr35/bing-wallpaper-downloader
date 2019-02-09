import {exec, execSync} from 'child_process';
import {existsSync, statSync} from 'fs';

class WallpaperService {


  setWallpaper(photoPath: string): Promise<boolean> {
    return new Promise<boolean>((resolve, reject) => {

      const key = this.isCinnamon() ? 'org.cinnamon.desktop.background' : 'org.gnome.desktop.background';

      exec(`gsettings set ${key} picture-uri file://${photoPath}`, (error, _stdout, _stderr) => {
        return !error ? resolve(true) : reject(error)
      });
    });
  }


  isWallpaperAlreadySet(photoPath: string): boolean {
    const exists = existsSync(photoPath);
    if (exists) {
      const stats = statSync(photoPath);
      stats.birthtime.setHours(0);
      stats.birthtime.setMinutes(0);
      return (new Date().getTime() - stats.birthtime.getTime()) < (24 * 60 * 60 * 1000); // image is older than 24h
    }
    return false;
  }


  isCinnamon(): boolean {
    try {
      return execSync('gsettings writable org.cinnamon.desktop.background picture-uri').toString('utf8').indexOf('true') !== -1;
    } catch (e) {
      return false;
    }
  }

}

export default new WallpaperService();
