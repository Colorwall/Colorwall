<div align="center">
<img width="1920" height="1077" alt="Image" src="https://github.com/user-attachments/assets/fd8574e6-3f1c-4386-9a81-0332f43f8bce" />
<img width="1536" height="1024" alt="LxColorWall" src="https://github.com/user-attachments/assets/a0455f0b-c580-4904-a8a1-da9680bf16d3" />
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/1ebc62a7-b1ec-4e4d-9264-f14f325dbf46" />

---

# ColorWall

[DOES THY WANT TO GIVE FEEDBACK? HERE <.3](https://www.colorwall.xyz/feedback)

## https://www.colorwall.xyz

### A Highly Performant Wallpaper Engine Built in Rust/Tauri, No bloat, Desktop costumization In One place for no cost.

**Designed for Performance • Ease of Usage • Optimization Enthusiasts • People who don't want thier pc to die • From Igpu tp RTX gpus offering presets and twio *Renderers* as per liking**

Under Dev

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat-square&logo=tauri&logoColor=black)](https://tauri.app)
[![Release](https://img.shields.io/github/v/release/Colorwall/Colorwall?style=flat-square&color=blue)](https://github.com/Colorwall/Colorwall/releases)
[![License](https://img.shields.io/github/license/Colorwall/Colorwall?style=flat-square)](LICENSE)

> **3–8% GPU Usage on a low end IGPU • 4KVsync Video Wallpapers • 9+ Auto-Scraped Sources • Perf/Ultra Rendering Engines**
>
> *No infrastructure required, the application runs locally and works fully offline if u keep stuff in store or upload your own, Store will not work when offline*

[Download](#-installation)

### And to anyone wondering, this project is made alone by @LaxentaInc, it's been almost 11 months making this app which started as just a way to compete with my gf's ArchiveTune project <.3 (we broke up)

</div>

---

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
- Mpv with HIGH res codecs with EWA lancing and frame interpolation to give the best output if you have the gpu for it
- Free integrated wallpaper store
- Taskbar configuration options
- Manual wallpaper uploads supported
- Interactive wallpapers with music supports.
- Clear and accessible settings
- Multi Monitor support
- Proper thread and access violation fixes (we use multithreading now, simple flag), along with the best possible ui unmounting i knew, to give the best ui i could and also make it take no resources when it's hidden in systray <.3
- Automatic power-saving: pauses video when windows are maximized or focused
- **Unified Search**: retrieve results from all sources in a single query

---

## Installation

### Windows

1. Download `ColorWall-Setup.exe` from the [Latest Release](https://github.com/Colorwall/Colorwall/releases/latest).
2. Run the installer.

> **SmartScreen Notice:**
> If a "Windows protected your PC" message appears, select **More info → Run anyway**.
> The application is not code-signed due to certificate costs. 

---

<img src="https://komarev.com/ghpvc/?username=LaxentaInc&color=00B4D8&style=flat-square"/>
