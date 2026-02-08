<div align="center">
  
![Preview](thecolorwall.png)
---
# ColorWall

### A Free Wallpaper Engine Alternative Built in Rust  
**Designed for Performance • Optimization Enthusiasts • Everyday Users • Low-Spec PCs**



[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat-square&logo=tauri&logoColor=black)](https://tauri.app)
[![Release](https://img.shields.io/github/v/release/laxenta/WallpaperEngine?style=flat-square&color=blue)](https://github.com/laxenta/WallpaperEngine/releases)
[![License](https://img.shields.io/github/license/laxenta/WallpaperEngine?style=flat-square)](LICENSE)

> **~0.5% CPU / 3–8% GPU • 4K Video Wallpapers • 6+ Auto-Scraped Sources**  
>
> *No infrastructure required — the application runs locally and works fully offline.*

[Download](#-installation) • [✨ Features](#-features) • [Screenshots](#-screenshots) • [Build](#-build-from-source)



</div>

---

## 📂 Source Code Status

ColorWall is currently closed-source as in today's world of A.I, anyone can copy a working logic and change the front-end. 

Publishing to the Microsoft Store requires a developer license and related costs, which we are currently self-funding. To ensure sustainability and proper release management, we plan to open-source the project once distribution infrastructure is secured.

If you would like to support the project and help accelerate open-sourcing efforts, you can contribute here:

👉 https://ko-fi.com/colorwall
Community support directly helps with licensing, signing certificates, and long-term development.

Transparency and community collaboration remain long-term goals of this project.


---

## 🚀 Why ColorWall?

- **Wallpaper Engine** requires a paid license and relies on the Steam Workshop ecosystem.
- **Lively Wallpaper** is free but may consume significantly more GPU resources.
- **ColorWall** is free, optimized for performance, and automatically aggregates art and wallpapers from multiple sources — minimizing manual effort.

### Performance Benchmark (Single Snapshot Test)

| App | CPU Usage | GPU Usage | Memory | Price |
| :--- | :---: | :---: | :---: | :---: |
| **ColorWall** | **0.7%** | **11.5%** | **316 MB** | **Free** |
| Lively Wallpaper | 1.9% | 74% | 294 MB | Free |
| Wallpaper Engine | 0.9% | 61.8% | 322 MB | $4 |

*Tested on an Intel i3 laptop with integrated graphics.*

---

## ✨ Features

### Live Video Wallpapers

- **4K 60fps** video wallpapers at approximately **0.5% CPU / 1–2% GPU** (hardware dependent)
- Uses **Windows Media Foundation** for native decoding and optimized performance
- Smooth playback even on lower-end hardware
- Free integrated wallpaper store
- Taskbar configuration options
- Manual wallpaper uploads supported
- Clear and accessible settings
- Automatic power-saving: pauses video when windows are maximized or focused
- **Unified Search**: retrieve results from all sources in a single query

---

### Intelligent Three-Tier Loading System

Designed for performance and bandwidth efficiency:

1. **Thumbnails** – Instant load (4–5 MB for 100 wallpapers)
2. **Previews** – Loaded on click (1–3 MB, instant playback)
3. **4K Download** – Only downloaded upon confirmation (cached locally)

**Result:** Up to 95% lower bandwidth usage compared to traditional wallpaper applications.

---

## Installation

### Windows

1. Download `ColorWall-Setup.exe` from the [Latest Release](https://github.com/laxenta/WallpaperEngine/releases/latest).
2. Run the installer.

> **SmartScreen Notice:**  
> If a “Windows protected your PC” message appears, select **More info → Run anyway**.  
> The application is not code-signed due to certificate costs. The project is fully open source.

---

### Linux & macOS

- **Linux:** Currently untested
- **macOS:** Not supported yet
- Additional platform support is planned

<div align="center">

| Platform | Download |
| :--- | :--- |
| ![Windows](assets/windows.PNG) **Windows** | [Download .exe](https://github.com/colorwall/colorwall/releases/latest) |

[📦 View All Releases](https://github.com/colorwall/colorwall/releases)

</div>

---

## Screenshots

<div align="center">

### Static Wallpaper Store  
![Gallery](assets/static.PNG)

### Live Wallpaper Store  
![Filters](assets/nofeet.PNG)

### Live Preview Modal with Video Player  
![Preview](assets/updatedmodal.PNG)

</div>




