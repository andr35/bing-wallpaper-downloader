# National Geographic Wallpaper

This script download the National Geographic Photo of the Day and 
set it as wallpaper.

## How to

Build: 
``` bash
$ npm run build
```
Run: 
``` bash
$ npm start # or
$ node dist/index.js
```

## Autostart

To run the script at every login, write this line in `~/.zlogin` file:
``` bash
node <PROJECT_PATH>/dist/index.js > /dev/null &
```
