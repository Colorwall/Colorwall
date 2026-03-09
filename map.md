# src-tauri Code Map

## Structure Overview
```
src-tauri/src/
├── main.rs              # Entry point - Tauri app setup, system tray, invoke handlers
├── lib.rs               # Library exports
│
├── core/                # Core app logic
│   ├── player/
│   │   └── state_machine.rs    # Video wallpaper state, create/stop/status
│   ├── ipc/
│   │   └── channel.rs          # Process management (was process_manager.rs)
│   └── lifecycle/
│       ├── startup.rs          # Wallpaper restoration on app start
│       └── shutdown.rs         # Shutdown placeholder
│
├── ui/commands/         # Tauri command handlers (frontend calls these)
│   ├── search_ops.rs    # search_wallpapers, fetch_live2d, resolve_* commands
│   ├── wallpaper_ops.rs # set_wallpaper, video wallpaper commands
│   └── settings_ops.rs  # get_settings, save_settings, validate_mpv
│
├── data/
│   ├── models/          # Data structures
│   │   ├── settings.rs      # AppSettings struct
│   │   ├── wallpaper.rs     # Wallpaper, VideoWallpaperState structs
│   │   └── responses.rs     # SearchResult, WallpaperSource etc
│   ├── storage/
│   │   └── paths.rs         # get_wallpaper_dir, get_state_file etc
│   └── scrapers/        # Website scrapers (was scraper.rs, now split)
│       ├── wallhaven.rs
│       ├── wallpapers_com.rs
│       ├── wallpaperflare.rs
│       ├── moewalls.rs
│       ├── motionbgs.rs
│       ├── wallpaperwaifu.rs
│       └── utils.rs         # Common scraper helpers
│
├── platform/
│   ├── windows/         # Player binary standalone modules
│   │   ├── player_main.rs       # Entry point for wallpaper-player.exe
│   │   ├── player_wmf.rs        # WMF player implementation
│   │   ├── player_mpv.rs        # MPV player implementation
│   │   ├── player_injection.rs  # Desktop injection (behind icons)
│   │   └── player_os_version.rs # Windows version detection
│   └── linux/
│       └── video_wallpaper_linux.rs
│
└── utils/               # (placeholder, mostly empty)
    └── mod.rs
```

## Where Old Files Went

| Old File | New Location |
|----------|--------------|
| `models.rs` | Split → `data/models/settings.rs`, `wallpaper.rs`, `responses.rs` |
| `scraper.rs` | Split → `data/scrapers/` (7 files) |
| `storage.rs` | → `data/storage/paths.rs` |
| `video_wallpaper.rs` | Split → `core/player/state_machine.rs` + `core/lifecycle/startup.rs` |
| `process_manager.rs` | → `core/ipc/channel.rs` |
| `commands/search.rs` | → `ui/commands/search_ops.rs` |
| `commands/settings.rs` | → `ui/commands/settings_ops.rs` |
| `commands/wallpaper.rs` | → `ui/commands/wallpaper_ops.rs` |
| `windows/main.rs` | → `platform/windows/player_main.rs` |
| `windows/wmf_player.rs` | → `platform/windows/player_wmf.rs` |
| `windows/mpv_player.rs` | → `platform/windows/player_mpv.rs` |
| `windows/desktop_injection.rs` | → `platform/windows/player_injection.rs` |
| `windows/os_version.rs` | → `platform/windows/player_os_version.rs` |
| `linux/video_wallpaper_linux.rs` | → `platform/linux/video_wallpaper_linux.rs` |

## Binary Targets (Cargo.toml)

- **wallpaperengine** → `src/main.rs` (main Tauri app)
- **wallpaper-player** → `src/platform/windows/player_main.rs` (standalone player process)
