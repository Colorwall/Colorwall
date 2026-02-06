<div align="center">

# ColorWall

### Free Wallpaper Engine Alternative Built in Rust
**For Performance • Optimization Connoisseurs • Normal Users • And potato PCs**

> Contributions are heavily welcome! Im open to learning the mistakes i might have made, im not the best at rust or Systems programming for multiple OSs (PLEASE open issues for anything i didn't test. This is literally in its development stage, Contributions are welcome!)

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=flat-square&logo=tauri&logoColor=black)](https://tauri.app)
[![Release](https://img.shields.io/github/v/release/laxenta/WallpaperEngine?style=flat-square&color=blue)](https://github.com/laxenta/WallpaperEngine/releases)
[![License](https://img.shields.io/github/license/laxenta/WallpaperEngine?style=flat-square)](LICENSE)

> **~0.5% CPU / 3-8% GPU • 4K Video Wallpapers • 6+ Auto-Scraped Sources**
>
> *Zero Infrastructure Needed - Just the app and your pc becomes the host. Works Offline.*

[Download](#-installation) • [✨ Features](#-features) • [Screenshots](#-screenshots) • [Build](#-build-from-source)

![Preview](assets/Capture.PNG)

</div>

---

## 🚀 Why ColorWall?

> **Wallpaper Engine** costs $4 and locks you into the Steam Workshop which is also paid sometimes
> **Lively Wallpaper** is free but can be heavier on resources (uses 6x more GPU)
> **ColorWall** is **free**, **faster**, and scrapes All kind of ART/Wallpapers automatically, so no overhead needed from user

### Performance Benchmark on my old laptop at one time snapshot

| App | CPU Usage | GPU Usage | Memory | Price |
| :--- | :---: | :---: | :---: | :---: |
| **ColorWall** | **0.7%** | **11.5%** | **316 MB** | **Free** |
| Lively Wallpaper | 1.9% | 74% | 294 MB | Free |
| Wallpaper Engine | 0.9% | 61.8% | 322 MB | $4 |

*Tested on an Intel i3 integrated graphics laptop - if it works there, it'll on your PC*

---

## ✨ Features

### Live Video Wallpapers
- **4K 60fps** video wallpapers at **~0.5% CPU / ~1-2% GPU** (depends on hardware)
- Uses **Windows Media Foundation** (We use Windows Media Foundation by default for native decoded performance and quick sync for performance, instead of using better video encoders like mpv :c)
- Smooth playback even on potato PCs.
- Free and open store for setting wallpapers without a hassle.
- Taskbar configurations
- HUGE store thats absolutely free to use and the app is overall personalized for user experience haha!
- Manual Uploads Of your favorite wallpapers!
- Clear settings for everything!
- Auto powersaving by pausing video when a window is maximized or in focus mode.
- **Unified Store**: Search once, get results FROM our Store, Just a search away/

### Smart 3 turn Loading- Keeping in mind for perfomance and metered (most ppl have wifi tbh)
1. **Thumbnails**: Load instantly (4-5 MB for 100 wallpapers).
2. **Previews**: On click (1-3 MB, instant playback).
3. **4K Download**: only when you confirm (gets cached)

**Result:** 95% less bandwidth usage than traditional wallpaper apps.

---

## Installation ✨✨✨

### Windows
1. Download `ColorWall-Setup.exe` from the [Latest Release](https://github.com/laxenta/WallpaperEngine/releases/latest).
2. Run the installer.
   > **Note on SmartScreen:**  
   > If you see a "Windows protected your PC" popup, click **More info** → **Run anyway**.  
   > *App isn't code-signed because certificates cost hundreds of dollars. It's open source!*
   >
   > Not that the opinion of windows defender matters, thats dogshit anyways, doesn't even help with real malwares lmao

### Linux & macOS
- **Linux**: UNTESTED.
- **macOS**: NOT SUPPORTED YET
- Other ones coming soon ;c even android will be there when i get the time to do it

<div align="center">

| Platform | Download |
| :--- | :--- |
| 🪟 **Windows** | [Download .exe](https://github.com/laxenta/WallpaperEngine/releases/latest) |
| 🐧 **Linux** | [Download .AppImage](https://github.com/laxenta/WallpaperEngine/releases/latest) |
<!-- | 🍎 **macOS** | [Download .dmg](https://github.com/laxenta/WallpaperEngine/releases/latest) | -->

[📦 View All Releases](https://github.com/laxenta/WallpaperEngine/releases)

</div>

---

## Screenshots

<div align="center">

### Static Wallpaper Store, Free obviously, The whole app is free/transparent
![Gallery](assets/static.PNG)

### Live wallpaper store (soon adding 2 more planned video sources)
![Filters](assets/nofeet.PNG)

### Live Preview Modal with Video Player
![Preview](assets/updatedmodal.PNG)

</div>

---

## Build from Source

For developers or those who prefer building themselves (Only if you don't trust the releases or want to contribute):

```bash
# Clone repo
git clone https://github.com/laxenta/WallpaperEngine.git
cd WallpaperEngine
# You can uh use npm or yarn or whatever u like man it doesn't matter
# Install dependencies
pnpm install

# Run in development
pnpm tauri dev

# Build for production
pnpm tauri build
```

**Requirements:** Windows/Linux/macOS, Node.js 18+, pnpm, Rust 1.70+

---

## 🤝 Contributing

Welcome! Ideas:

- [ ] Suggest more sources
- [ ] Favorites/collections/Upload system
- [x] System tray icon
- [ ] Auto-change wallpaper on timer
- [ ] Fix Niche or bugs or Suggest improvements PLS
- [ ] Linux Wayland additional support
- [ ] Mobile (Android via Tauri Mobile)
- Or just sponsor this project, you will be the coolest cutie in the world

See [Issues](https://github.com/laxenta/WallpaperEngine/issues) for more!!

---

## 🌍 Platform Support

| Platform | Status |
| :--- | :--- |
| **Windows 10/11** | Already supported |
| **Linux (X11)** | Jan 2026 |
| **Linux (Wayland, KDE)** | NEXT TARGET |
| **Android** | Planned After Linux |
| **macOS** | If asked for |
| **iOS** | Unlikely |

---

## 💖 Support This Project

If this saved you $4 and your GPU:

- ⭐ **Star the repo** (it matters!)
- Report bugs (very important bruh)
- Suggest features
- [Sponsor](https://github.com/sponsors/laxenta) (helps fund Android/iOS ports)

---
![me](assets/me.jpg)

<div align="center">

**Built with ❤️ by [@laxenta, @laxenta.me](https://github.com/laxenta)**

[Laxenta Inc](https://laxenta.tech) • [Website](https://laxenta.tech) • [Issues](https://github.com/laxenta/WallpaperEngine/issues)
---
![Views](https://hits.sh/github.com/laxenta/WallpaperEngine.svg?style=for-the-badge&label=Views&color=blue)
---

*Made because a Wallpaper Engine doesn't need to cost that much MONEY and have so many random purchases or use tons of CPU/GPU*

</div>