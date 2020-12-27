# Bing Wallpaper Downloader

Simple CLI application written in [Rust](https://www.rust-lang.org/) to download
the Bing Image of the day and, optionally, set it as wallpaper or show a desktop
notification with the image and its caption.

## Build

The build phase requires `libdbus-1-dev` as dependency:

```bash
sudo apt-get install libdbus-1-dev
```

Build:

```bash
cargo build          # Debug version
cargo build --relase # Release version
```

## How to use

Read the help message:

```bash
./bing_wallpaper_downloader download --help 
```

Download the image of the day in a specific folder. `-o` arg is optional. If it
is not provided the image will be saved in a predefined folder:

```bash
./bing_wallpaper_downloader download -o $HOME/Downloads/myimage.jpg 
```

Download image of a previous day, e.g., the image of 2 days ago:

```bash
./bing_wallpaper_downloader download -o $HOME/Downloads/myimage.jpg -p 2
```

Download the image of tha day and set it as wallpaper:

```bash
./bing_wallpaper_downloader download -s
```

Show a desktop notification with the image of the day and the caption:

```bash
./bing_wallpaper_downloader download -n
```

## Autostart

If you use `zsh`, to run the application at login and see a desktop notification
with the new photo of the day, append this line to the `~/.zlogin` file:

```bash
<PROJECT_PATH>/target/release/bing_wallpaper_downloader -n > /dev/null &
```

## Development

The project contains the configuration to run a [VSCode developer
container](https://code.visualstudio.com/docs/remote/containers) that can be
used to quicky setup the development environment. See
[here](https://code.visualstudio.com/docs/remote/containers) for more
information.

## Versions

Different versions of this app written in different languages.

| Language | Link                                                                                                 |
| -------- | ---------------------------------------------------------------------------------------------------- |
| Python   | <https://github.com/Andr35/NG-Wallpaper> (NB: it downloads the National Geographic Photo of the Day) |
| Node     | <https://github.com/Andr35/national-geographic-wallpaper/tree/node>                                  |
| Rust     | <https://github.com/Andr35/bing-wallpaper-downloader>                                                |

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
