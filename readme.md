<div align="center">
  
<img width="1536" height="1024" alt="LxColorWall" src="https://github.com/user-attachments/assets/a0455f0b-c580-4904-a8a1-da9680bf16d3" />

---
# ColorWall
[colorwall.xyz](https://colorwall.xyz)

### A Highly Performant Wallpaper Engine Built in Rust/Tauri, No bloat, Desktop costumization In One place for no cost.
**Designed for Performance • Ease of Usage • Optimization Enthusiasts • People who don't want thier pc to die • From Igpu tp RTX gpus offering presets and twio *Renderers* as per liking**
Under Dev 


[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat-square&logo=tauri&logoColor=black)](https://tauri.app)
[![Release](https://img.shields.io/github/v/release/Colorwall/Colorwall?style=flat-square&color=blue)](https://github.com/laxenta/WallpaperEngine/releases)
[![License](https://img.shields.io/github/license/Colorwall/Colorwall?style=flat-square)](LICENSE)

> **3–8% GPU Usage on a low end IGPU • 4KVsync Video Wallpapers • 9+ Auto-Scraped Sources • Perf/Ultra Rendering Engines**  
>
> *No infrastructure required, the application runs locally and works fully offline if u keep stuff in store or upload your own, Store will not work when offline*

[Download](#-installation) • [✨ Features](#-features) • [Screenshots](#-screenshots) • [Build](#-build-from-source)



</div>

---

## Source Code Status
ColorWall is currently closed-source bcz we don't need some looser to steal it and rebrand it to "Wallpaper Pro" and sell it, anyone can copy a working logic and change the front-end. 
We will be devloping another Desktop Modification Engine for linux and another app for android and multiple services, so we need time. Respectfully, fuck off to vibe coders who i know will copy this in a heartbeat.
So  we want to ensure that we establish our certificates and **recoganization** first, so Publishing to the Microsoft Store, steam etc. requires a developer license and related costs. 
## We are not fucking Jesus to not want anything for providing a whole desktop costumization engine, the least we deserve is recoganization, 
### code will be open source, but u HAVE TO WAIT MAN. We not rich to get certificates and register everything instantly, like i am literally 18 brother, studying to be a cop and in collage too >.<
- To ensure sustainability and proper release management, we plan to open-source the project once distribution infrastructure is secured.

If you would like to support the project and help accelerate open-sourcing efforts, you can contribute here:

👉 https://ko-fi.com/laxenta
Community support directly helps with licensing, signing certificates, and long-term development.

Transparency and community collaboration remain long-term goals of this project.


---

## 🚀 Why ColorWall?

- **Wallpaper Engine** requires a paid license and relies on the Steam Workshop ecosystem.
- **Lively Wallpaper** is free but may consume significantly more GPU resources.
- **ColorWall** is free, optimized for performance, has a sourced store, and we don't store any of those wallpapers, we also offer multiple Renderers/ Perfromance presets! <.3

### Performance Benchmark (Single Snapshot Test)

| App | CPU Usage | GPU Usage | Memory | Price |
| :--- | :---: | :---: | :---: | :---: |
| **ColorWall** | **0.7%** | **11.5%** | **316 MB** | **Free** |
| Lively Wallpaper | 1.9% | 74% | 294 MB | Free |
| Wallpaper Engine | 0.9% | 61.8% | 322 MB | $4 |

*Tested on an Intel i3 laptop with integrated graphics.*

---

## Features

### Live Video Wallpapers

- **4K Vsync** video wallpapers at approximately **0.5% CPU / 1–10% GPU** (Uses direct gpu decode instead of going through an diffrent pipeline and then going to gpu, direct d3d11 access cz IMF ofcourse)
- Uses **Windows Media Foundation** for native decoding and optimized performance
- Smooth playback even on lower-end hardware
- Free integrated wallpaper store
- Taskbar configuration options
- Manual wallpaper uploads supported
- Interactive wallpapers with music supports.
- Clear and accessible settings
- Multi Monitor support
- Mpv with HIGH res codecs with EWA lancing and frame interpolation to give the best output if you have the gpu for it.
- Proper thread and access violation fixes with the best possible ui unmounting i knew, to give the best ui i could and also make it take no resources when it's hidden in systray <.3
- Automatic power-saving: pauses video when windows are maximized or focused
- **Unified Search**: retrieve results from all sources in a single query


---

## Installation

### Windows

1. Download `ColorWall-Setup.exe` from the [Latest Release]([https://github.com/Colorwall/Colorwall/releases/latest).
2. Run the installer.

> **SmartScreen Notice:**  
> If a “Windows protected your PC” message appears, select **More info → Run anyway**.  
> The application is not code-signed due to certificate costs. The project is fully open source.

---




<img src="https://komarev.com/ghpvc/?username=LaxentaInc&color=00B4D8&style=flat-square"/>


