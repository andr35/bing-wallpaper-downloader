# National Geographic Wallpaper

This simple script downloads the National Geographic Photo of the Day and 
set it as wallpaper.

## Optional arguments:

#### show-notification

Use `node dist/index.js show-notification` to create a notification having as 
icon the new downloaded image without applying is as wallpaper.

#### force-download

Use `node dist/index.js force-download` to force the re-download of the photo if 
it was previously downloaded.

#### prev-day n

Use `node dist/index.js prev-day n` to request the photo of the day of a backward day
(eg: `node dist/index.js prev-day 1` will download the photo of yesterday).
__Only the photos of the current month are available.__

NB: The 3 arguments can be used togheter.

## How to

- Build:

  Compile typescript sources in `dist/` folder.
  ``` bash
  $ npm run build
  ```

- Packaging:
  
  Create standalone executables in `out/` folder.
  ``` bash
  $ npm run package
  ```

- Run:
  ``` bash
  $ npm start # or
  $ node dist/index.js # or (if packaged)
  $ ./out/index-linux
  ```

## Autostart

To run the script at every login, write this line in `~/.zlogin` file:
``` bash
node <PROJECT_PATH>/dist/index.js > /dev/null &
```
