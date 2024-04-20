# Discord Data Parser

A cross-platform application that can parse the discord data package through a GUI, making it easier to examine through it.
[What is in my data package?](https://support.discord.com/hc/en-us/articles/360004957991-Your-Discord-Data-Package)


## How to use

Have you have your package.zip ready. [My what?](https://support.discord.com/hc/en-us/articles/360004027692-Requesting-a-Copy-of-your-Data)

While the application is open, click on the "Open Package" button and select the package.zip you received from Discord.

You can now browse through the app and see what your data holds!

## Build

`git clone https://github.com/genki-ai/discord-data-package-reader.git` 

`bun install`

`bunx tauri build`

## TODO

- [ ] Cleanup UI
- [ ] Show profile picture
- [ ] Message search
- [ ] Analytics
- [ ] Export data
- [ ] Saved gifs
- [ ] Group channels into their servers
- [ ] Friends list
- [ ] Show usernames instead of ID for DM messages


## Built with

- [Tauri](https://github.com/tauri-apps/tauri) - Cross platform application
- [Bun](https://github.com/oven-sh/bun) - Javascript runtime
- [Rust](https://github.com/rust-lang/rust) - Backend
- my tiny paws
