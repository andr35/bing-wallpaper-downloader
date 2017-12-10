# National Geographic Wallpaper

This script download the National Geographic Photo of the Day and 
set it as wallpaper.

## Optional arguments:

#### show-notification

Use `node dist/index.js show-notification` to create a notification having as icon the new downloaded image without applying is as wallpaper.

#### force-download

Use `node dist/index.js force-download` to force the re-download of the photo if it was previously downloaded.

NB: The 2 arguments can be used togheter.

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
