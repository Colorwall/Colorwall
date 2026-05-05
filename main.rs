// prevents console window on windows in release builds
// main tauri entry point for loading everything together
// saving on github for reference.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};
use wallpaperengine::breadcrumb_data;
use wallpaperengine::core::lifecycle::restore_wallpaper_on_startup;
use wallpaperengine::core::player::app_management::start_renderer_watchdog;
use wallpaperengine::core::player::manager::shutdown_video_wallpaper;
use wallpaperengine::core::player::state::periodic_state_save;
use wallpaperengine::core::telemetry::{
    add_breadcrumb, capture_error, init_sentry, wait_for_system_ready, BreadcrumbType,
};
use wallpaperengine::platform::windows::interactive::watchdog::start_interactive_watchdog;
use wallpaperengine::ui::commands::*;

fn main() {
    // Check if running in autostart mode (silently from system tray) - Will be diffrent for linux i think :)
    let args: Vec<String> = std::env::args().collect();
    let is_autostart = args
        .iter()
        .any(|arg| arg == "--autostart" || arg == "/autostart");
    let _guard = init_sentry();

    add_breadcrumb(
        BreadcrumbType::Lifecycle,
        "Application starting",
        breadcrumb_data!("autostart" => is_autostart),
    );

    if is_autostart {
        println!("[main] Starting in autostart mode (tray only)");
    } else {
        println!("[main] Starting in normal mode (show window)");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_single_instance::init(|app, _argv, _cwd| {
                // when a second instance is launched, focus the existing window
                println!("[main] Second instance detected, focusing existing window");
                
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.unminimize();
                    println!("[main] Existing window focused");
                } else {
                    println!("[main] Window doesn't exist, recreating...");
                    use tauri::{WebviewUrl, WebviewWindowBuilder};
                    let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                        .title("ColorWall - Wallpaper Engine")
                        .inner_size(1000.0, 900.0)
                        .resizable(true)
                        .decorations(false)
                        .transparent(true)
                        .build();
                    
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            })
        )
        .invoke_handler(tauri::generate_handler![
            search_wallpapers,
            fetch_live2d,
            resolve_wallpaperflare_highres,
            resolve_motionbgs_video,
            resolve_wallpaperwaifu_video,
            resolve_wallpapersclan_highres,
            resolve_desktophut_video,
            resolve_konachan_highres,
            autocomplete_tags,
            get_cached_tag_count,
            set_wallpaper,
            get_current_wallpaper,
            get_cache_size,
            clear_cache,
            set_video_wallpaper,
            stop_video_wallpaper_command,
            get_video_wallpaper_status,
            list_user_wallpapers,
            upload_user_wallpaper,
            delete_user_wallpaper,
            register_local_wallpaper,
            set_local_wallpaper,
            set_local_video_wallpaper,
            get_wallpaper_storage_path,
            download_wallpaper,
            download_to_library,
            is_in_library,
            get_settings,
            save_settings,
            validate_mpv_path,
            get_startup_enabled,
            set_startup_enabled,
            get_username,
            set_discord_rpc_window_focus,
            get_system_info,
            get_monitors,
            toggle_monitor_wallpaper,
            get_active_monitors,
            configure_taskbar,
            set_window_vibrancy,
            download_homepage_asset,
            get_monitor_wallpaper_info,
            set_video_wallpaper_on_monitors,
            check_for_updates,
            install_update,
            list_interactive_wallpapers,
            import_interactive_wallpaper,
            set_interactive_wallpaper,
            stop_interactive_wallpaper,
            get_interactive_properties,
            update_interactive_property,
            delete_interactive_wallpaper,
            download_and_setup_mpv,
            check_mpv_installed,
            download_interactive_assets,
            resync_interactive_assets,
            check_interactive_assets_installed,
            check_interactive_assets_downloading,
            list_widgets,
            get_widget_config,
            save_widget_config,
            import_widget,
            delete_widget,
            get_global_widgets,
            spawn_widget_on_desktop,
            remove_widget_from_desktop,
            kill_all_widgets,
            save_global_widgets,
            update_widget_position,
            cancel_library_download,
            cw_list_scenes,
            cw_create_scene,
            cw_load_scene,
            cw_save_scene,
            cw_start_scene,
            cw_stop_scene,
            cw_capture_preview,
            cw_delete_scene,
            cw_import_video,
            cw_get_scenes_dir,
            cw_update_layer,
            cw_list_shader_presets,
            cw_list_particle_presets,
            cw_export_scene,
            cw_save_layer_mask,
            cw_spawn_studio,
            cw_close_studio,
        ])
        .setup(move |app| {
            add_breadcrumb(BreadcrumbType::Lifecycle, "Tauri app setup started", None);
            wallpaperengine::core::discord_rpc::init_discord_rpc();
            let window = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            {
                let app_handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    // Small delay to ensure window is ready
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    
                    // Check settings and apply vibrancy if enabled
                    if let Ok(response) = get_settings().await {
                        if let Some(settings) = response.settings {
                            if settings.window_vibrancy {
                                let _ = set_window_vibrancy(app_handle, true);
                            }
                        }
                    }
                });
            }

            let _app_handle = app.handle().clone();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    // Prevent the window from being totally destroyed
                    api.prevent_close();
                    
                    // The App.tsx knows when the window is hidden and will unmount the React DOM 
                    // completely to drop GPU usage to 0%.
                    let _ = window_clone.hide();
                    
                    add_breadcrumb(BreadcrumbType::UI, "Window close requested (hidden to tray)", None);
                    println!(
                        "[main] Close button clicked - UI hidden to tray, wallpaper continues in background"
                    );
                }
            });

            // systray
            add_breadcrumb(BreadcrumbType::UI, "Initializing system tray", None);
            
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::{MouseButton, TrayIconBuilder};
            let show_item =
                MenuItem::with_id(app, "show", "Show Window", true, None::<&str>).unwrap();
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&show_item, &quit_item]).unwrap();

            let app_handle_for_tray = app.handle().clone();
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        add_breadcrumb(BreadcrumbType::UI, "Show window from tray menu", None);
                        
                        if app.get_webview_window("main").is_none() {
                            use tauri::{WebviewUrl, WebviewWindowBuilder};
                            let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                                .title("ColorWall - Wallpaper Engine")
                                .inner_size(1000.0, 900.0)
                                .resizable(true)
                                .decorations(false)
                                .transparent(true)
                                .build();
                        }

                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            
                            // REAPPLY VIBRANCY SETTINGS TO ENSURE THEY ARE ACTIVE AFTER SHOWING
                            // IDK IF THIS IS IMPORTANT OR REDUNDANT TBH BUT I THINK IT IS NOTHING BAD [are we are duping? i will fix it later]
                            #[cfg(target_os = "windows")]
                            {
                                let app_handle = app.clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Ok(response) = get_settings().await {
                                        if let Some(settings) = response.settings {
                                            if settings.window_vibrancy {
                                                let _ = set_window_vibrancy(app_handle, true);
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                    "quit" => {
                        add_breadcrumb(BreadcrumbType::Lifecycle, "Quit requested from tray", None);
                        
                        println!("[main] Quit requested from tray");
                        let _ = shutdown_video_wallpaper(&app_handle_for_tray);
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: MouseButton::Left,
                        ..
                    } = event
                    {
                        // Recreate
                        if tray.app_handle().get_webview_window("main").is_none() {
                            add_breadcrumb(BreadcrumbType::UI, "Recreating window from tray click", None);
                            
                            println!("[main] Window doesn't exist, recreating from tray");
                            use tauri::{WebviewUrl, WebviewWindowBuilder};
                            let _ = WebviewWindowBuilder::new(
                                tray.app_handle(),
                                "main",
                                WebviewUrl::default(),
                            )
                            .title("ColorWall - Wallpaper Engine")
                            .inner_size(1000.0, 900.0)
                            .resizable(true)
                            .decorations(false)
                            .transparent(true)
                            .build();
                        }

                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            println!("[main] Window shown from tray icon click");
                            
                            // REAPPLY VIBRANCY SETTINGS TO ENSURE THEY ARE ACTIVE AFTER SHOWING
                            // This fixes the white/light mode issue when restoring from tray
                            #[cfg(target_os = "windows")]
                            {
                                let app_handle = tray.app_handle().clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Ok(response) = get_settings().await {
                                        if let Some(settings) = response.settings {
                                            if settings.window_vibrancy {
                                                let _ = set_window_vibrancy(app_handle, true);
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                })
                .build(app)
                .unwrap();

            add_breadcrumb(BreadcrumbType::UI, "System tray initialized", None);

            // init Taskbar
            #[cfg(target_os = "windows")]
            {
                use wallpaperengine::platform::windows::os::taskbar::init_taskbar_keeper;
                init_taskbar_keeper();
                println!("[main] Taskbar keeper initialized");

                // enable autostart by default in production builds (first run)
                // if user later disables it in settings, that's respected
                #[cfg(not(debug_assertions))]
                {
                    use wallpaperengine::platform::windows::os::windows_startup;
                    if !windows_startup::is_startup_enabled() {
                        println!("[main] enabling autostart (production default)");
                        let _ = windows_startup::set_startup_enabled(true);
                    }
                }
            }

            // restore wallpaper on startup in background task
            let app_handle = app.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                add_breadcrumb(BreadcrumbType::Lifecycle, "Startup restoration task spawned", None);
                if let Ok(settings_res) = get_settings().await {
                    if let Some(settings) = settings_res.settings {
                        wallpaperengine::core::discord_rpc::apply_settings(
                            settings.discord_rpc_enabled,
                            settings.discord_custom_status,
                            settings.discord_custom_details,
                        );
                    }
                }
                
                // wait for system to be fully ready
                let system_ready = wait_for_system_ready(15).await;
                
                if !system_ready {
                    capture_error(
                        "System not ready after 15s - attempting restoration anyway: Possibly a VM? So slow bruh",
                        breadcrumb_data!(
                            "action" => "proceeding_anyway",
                            "risk" => "may_fail"
                        ),
                    );
                }
                
                // show window AFTER system ready but BEFORE restoration (only if normal launch)
                // this lets user see the loading screen during restoration
                if !is_autostart {
                    println!("[startup] Showing window with loading screen (normal launch)");
                    add_breadcrumb(BreadcrumbType::UI, "Showing window before restoration starts", None);
                    
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                    
                    // give React time to mount and render loading screen [should work, and not show transparent window]
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }

                println!("[startup] attempting wallpaper restoration");
                add_breadcrumb(BreadcrumbType::Lifecycle, "Starting wallpaper restoration", 
                    breadcrumb_data!("system_ready" => system_ready));
                
                match restore_wallpaper_on_startup(&app_handle).await {
                    Ok(_) => {
                        println!("[startup] restoration completed");
                        add_breadcrumb(BreadcrumbType::Lifecycle, "Wallpaper restoration completed", None);
                    }
                    Err(e) => {
                        eprintln!("[startup] error: failed to restore wallpaper: {}", e);
                        capture_error(
                            &format!("Startup wallpaper restoration failed: {}", e),
                            breadcrumb_data!(
                                "system_ready" => system_ready,
                                "error" => e.clone()
                            ),
                        );
                    }
                }
                
                // also try to restore global widgets
                #[cfg(target_os = "windows")]
                {
                    println!("[startup] attempting widget restoration");
                    wallpaperengine::platform::windows::interactive::widget_host::restore_global_widgets(&app_handle);
                }
                
                if is_autostart {
                    println!("[startup] Staying in tray (autostart mode)");
                    add_breadcrumb(BreadcrumbType::UI, "Staying in tray - autostart mode", None);
                }
            });

            // Periodic state saving to prevent data loss (every 30 seconds)
            // TODO: make this dynamic than hardcoded, but whatever for now, i focus on more imp things
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    periodic_state_save();
                }
            });

            // renderer watchdog — auto-restarts crashed wallpaper-player processes
            start_renderer_watchdog(app.handle().clone());
            start_interactive_watchdog(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
