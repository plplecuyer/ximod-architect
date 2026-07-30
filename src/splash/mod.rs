//! Native transparent splash screen implementation
//! 
//! Platform-specific implementations for Windows, Linux (X11), and macOS.
//! Creates a truly transparent borderless window that displays the splash image.

use std::time::{Duration, Instant};

/// Splash screen configuration
pub struct SplashConfig {
    pub image_path: std::path::PathBuf,
    pub display_duration: Duration,
    pub fade_duration: Duration,
    pub screen_width: f32,
    pub screen_height: f32,
}

/// Show the native transparent splash screen
/// Returns when the splash is complete
pub fn show_splash(config: SplashConfig) -> Result<(), String> {
    // Load the image first to get dimensions
    let image_data = std::fs::read(&config.image_path)
        .map_err(|e| format!("Failed to read splash image: {}", e))?;
    
    let image = image::load_from_memory(&image_data)
        .map_err(|e| format!("Failed to decode splash image: {}", e))?;
    
    let rgba = image.to_rgba8();
    let img_width = rgba.width();
    let img_height = rgba.height();
    
    // Calculate splash size (26% of screen width, max 1024, keep aspect ratio)
    let max_size = (config.screen_width * 0.26).min(1024.0);
    let scale = (max_size / img_width as f32).min(max_size / img_height as f32).min(1.0);
    let splash_width = (img_width as f32 * scale) as u32;
    let splash_height = (img_height as f32 * scale) as u32;
    
    // Center on screen
    let x = ((config.screen_width - splash_width as f32) / 2.0) as i32;
    let y = ((config.screen_height - splash_height as f32) / 2.0) as i32;
    
    tracing::info!(
        "Showing splash: {}x{} at ({}, {})",
        splash_width, splash_height, x, y
    );
    
    // Call platform-specific implementation
    #[cfg(target_os = "windows")]
    return windows::show_splash_window(&rgba, splash_width, splash_height, x, y, &config);
    
    #[cfg(target_os = "linux")]
    return linux::show_splash_window(&rgba, splash_width, splash_height, x, y, &config);
    
    #[cfg(target_os = "macos")]
    return macos::show_splash_window(&rgba, splash_width, splash_height, x, y, &config);
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        tracing::warn!("Native splash not supported on this platform, using fallback");
        std::thread::sleep(config.display_duration);
        Ok(())
    }
}

// ============================================================================
// Windows Implementation
// ============================================================================
#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::Graphics::Gdi::*;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    use windows_sys::Win32::System::LibraryLoader::*;
    use std::ptr;
    use std::mem;

    const WS_EX_LAYERED: u32 = 0x00080000;
    const WS_EX_TOPMOST: u32 = 0x00000008;
    const WS_EX_TOOLWINDOW: u32 = 0x00000080;
    const WS_POPUP: u32 = 0x80000000;
    const ULW_ALPHA: u32 = 0x00000002;
    const AC_SRC_OVER: u8 = 0x00;
    const AC_SRC_ALPHA: u8 = 0x01;
    const DIB_RGB_COLORS: u32 = 0;

    pub fn show_splash_window(
        rgba: &image::RgbaImage,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        config: &SplashConfig,
    ) -> Result<(), String> {
        unsafe {
            // Get module handle
            let hinstance = GetModuleHandleW(ptr::null());
            if hinstance.is_null() {
                return Err("Failed to get module handle".to_string());
            }

            // Register window class
            let class_name: Vec<u16> = "XimodSplash\0".encode_utf16().collect();
            
            let wc = WNDCLASSEXW {
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                style: 0,
                lpfnWndProc: Some(splash_wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: ptr::null_mut(),
                hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: ptr::null_mut(),
            };

            if RegisterClassExW(&wc) == 0 {
                // Class might already be registered, continue
            }

            // Create layered window
            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
                class_name.as_ptr(),
                ptr::null(),
                WS_POPUP,
                x, y,
                width as i32, height as i32,
                ptr::null_mut(), ptr::null_mut(), hinstance, ptr::null(),
            );

            if hwnd.is_null() {
                return Err("Failed to create splash window".to_string());
            }

            // Create compatible DC and bitmap
            let hdc_screen = GetDC(ptr::null_mut());
            let hdc_mem = CreateCompatibleDC(hdc_screen);
            
            // Create DIB section for the image
            let mut bmi: BITMAPINFO = mem::zeroed();
            bmi.bmiHeader.biSize = mem::size_of::<BITMAPINFOHEADER>() as u32;
            bmi.bmiHeader.biWidth = width as i32;
            bmi.bmiHeader.biHeight = -(height as i32); // Top-down
            bmi.bmiHeader.biPlanes = 1;
            bmi.bmiHeader.biBitCount = 32;
            bmi.bmiHeader.biCompression = BI_RGB;

            let mut bits: *mut std::ffi::c_void = ptr::null_mut();
            let hbitmap = CreateDIBSection(
                hdc_mem,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits,
                ptr::null_mut(),
                0,
            );

            if hbitmap.is_null() || bits.is_null() {
                DestroyWindow(hwnd);
                ReleaseDC(ptr::null_mut(), hdc_screen);
                DeleteDC(hdc_mem);
                return Err("Failed to create DIB section".to_string());
            }

            let old_bitmap = SelectObject(hdc_mem, hbitmap);

            // Scale and copy image data (RGBA to BGRA with premultiplied alpha)
            let scaled = image::imageops::resize(
                rgba,
                width,
                height,
                image::imageops::FilterType::Lanczos3,
            );
            
            let pixel_data = bits as *mut u8;
            for (i, pixel) in scaled.pixels().enumerate() {
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                let a = pixel[3] as u32;
                
                // Premultiply alpha for UpdateLayeredWindow
                let r_pm = ((r * a) / 255) as u8;
                let g_pm = ((g * a) / 255) as u8;
                let b_pm = ((b * a) / 255) as u8;
                
                let offset = i * 4;
                *pixel_data.add(offset) = b_pm;     // Blue
                *pixel_data.add(offset + 1) = g_pm; // Green
                *pixel_data.add(offset + 2) = r_pm; // Red
                *pixel_data.add(offset + 3) = a as u8; // Alpha
            }

            // Update layered window with full opacity initially
            update_layered_window(hwnd, hdc_screen, hdc_mem, x, y, width, height, 255);

            // Show window
            ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            UpdateWindow(hwnd);

            // Animation loop
            let start_time = Instant::now();
            let total_duration = config.display_duration + config.fade_duration;

            loop {
                // Process messages
                let mut msg: MSG = mem::zeroed();
                while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    if msg.message == WM_QUIT {
                        break;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                let elapsed = start_time.elapsed();
                
                if elapsed >= total_duration {
                    break;
                }

                // Calculate alpha for fade
                let alpha = if elapsed >= config.display_duration {
                    let fade_elapsed = elapsed - config.display_duration;
                    let fade_progress = fade_elapsed.as_secs_f32() / config.fade_duration.as_secs_f32();
                    ((1.0 - fade_progress) * 255.0) as u8
                } else {
                    255
                };

                // Update window alpha
                update_layered_window(hwnd, hdc_screen, hdc_mem, x, y, width, height, alpha);

                std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
            }

            // Cleanup
            SelectObject(hdc_mem, old_bitmap);
            DeleteObject(hbitmap);
            DeleteDC(hdc_mem);
            ReleaseDC(ptr::null_mut(), hdc_screen);
            DestroyWindow(hwnd);
            UnregisterClassW(class_name.as_ptr(), hinstance);

            Ok(())
        }
    }

    unsafe fn update_layered_window(
        hwnd: HWND,
        hdc_screen: HDC,
        hdc_mem: HDC,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        alpha: u8,
    ) {
        let mut pt_pos = POINT { x, y };
        let mut pt_src = POINT { x: 0, y: 0 };
        let mut size = SIZE { cx: width as i32, cy: height as i32 };
        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER,
            BlendFlags: 0,
            SourceConstantAlpha: alpha,
            AlphaFormat: AC_SRC_ALPHA,
        };

        unsafe {
            UpdateLayeredWindow(
                hwnd,
                hdc_screen,
                &mut pt_pos,
                &mut size,
                hdc_mem,
                &mut pt_src,
                0,
                &blend,
                ULW_ALPHA,
            );
        }
    }

    unsafe extern "system" fn splash_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }
}

// ============================================================================
// Linux (X11) Implementation
// ============================================================================
#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use x11::xlib::*;
    use std::ptr;
    use std::mem;
    use std::os::raw::*;

    pub fn show_splash_window(
        rgba: &image::RgbaImage,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
        config: &SplashConfig,
    ) -> Result<(), String> {
        unsafe {
            // Open display
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Failed to open X display".to_string());
            }

            let screen = XDefaultScreen(display);
            let root = XRootWindow(display, screen);
            let visual = XDefaultVisual(display, screen);
            let depth = XDefaultDepth(display, screen);

            // Try to find a 32-bit visual for transparency
            let mut vinfo: XVisualInfo = mem::zeroed();
            let has_argb = XMatchVisualInfo(display, screen, 32, TrueColor, &mut vinfo) != 0;
            
            let (use_visual, use_depth, colormap) = if has_argb {
                let cmap = XCreateColormap(display, root, vinfo.visual, AllocNone);
                (vinfo.visual, 32, cmap)
            } else {
                tracing::warn!("No 32-bit visual available, transparency may not work");
                (visual, depth, XDefaultColormap(display, screen))
            };

            // Window attributes
            let mut attrs: XSetWindowAttributes = mem::zeroed();
            attrs.colormap = colormap;
            attrs.border_pixel = 0;
            attrs.background_pixel = 0;
            attrs.override_redirect = True;

            let attr_mask = CWColormap | CWBorderPixel | CWBackPixel | CWOverrideRedirect;

            // Create window
            let window = XCreateWindow(
                display,
                root,
                x, y,
                width, height,
                0,
                use_depth,
                InputOutput as c_uint,
                use_visual,
                attr_mask,
                &mut attrs,
            );

            if window == 0 {
                XCloseDisplay(display);
                return Err("Failed to create X window".to_string());
            }

            // Scale image
            let scaled = image::imageops::resize(
                rgba,
                width,
                height,
                image::imageops::FilterType::Lanczos3,
            );

            // Create XImage
            let mut pixel_data: Vec<u32> = Vec::with_capacity((width * height) as usize);
            for pixel in scaled.pixels() {
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                let a = pixel[3] as u32;
                // ARGB format for X11
                pixel_data.push((a << 24) | (r << 16) | (g << 8) | b);
            }

            let ximage = XCreateImage(
                display,
                use_visual,
                use_depth as c_uint,
                ZPixmap,
                0,
                pixel_data.as_mut_ptr() as *mut c_char,
                width,
                height,
                32,
                0,
            );

            if ximage.is_null() {
                XDestroyWindow(display, window);
                XCloseDisplay(display);
                return Err("Failed to create XImage".to_string());
            }

            // Prevent XDestroyImage from freeing our data
            // We'll handle it ourselves
            
            // Create GC
            let gc = XCreateGC(display, window, 0, ptr::null_mut());

            // Map window
            XMapRaised(display, window);
            XFlush(display);

            // Draw initial image
            XPutImage(display, window, gc, ximage, 0, 0, 0, 0, width, height);
            XFlush(display);

            // Animation loop
            let start_time = Instant::now();
            let total_duration = config.display_duration + config.fade_duration;

            loop {
                // Process X events
                while XPending(display) > 0 {
                    let mut event: XEvent = mem::zeroed();
                    XNextEvent(display, &mut event);
                    
                    if event.type_ == Expose {
                        XPutImage(display, window, gc, ximage, 0, 0, 0, 0, width, height);
                        XFlush(display);
                    }
                }

                let elapsed = start_time.elapsed();
                
                if elapsed >= total_duration {
                    break;
                }

                // For fade effect on X11, we need to modify pixel alpha and redraw
                if elapsed >= config.display_duration {
                    let fade_elapsed = elapsed - config.display_duration;
                    let fade_progress = fade_elapsed.as_secs_f32() / config.fade_duration.as_secs_f32();
                    let alpha_mult = 1.0 - fade_progress;

                    // Update pixel data with faded alpha
                    for (i, pixel) in scaled.pixels().enumerate() {
                        let r = pixel[0] as u32;
                        let g = pixel[1] as u32;
                        let b = pixel[2] as u32;
                        let a = (pixel[3] as f32 * alpha_mult) as u32;
                        pixel_data[i] = (a << 24) | (r << 16) | (g << 8) | b;
                    }

                    XPutImage(display, window, gc, ximage, 0, 0, 0, 0, width, height);
                    XFlush(display);
                }

                std::thread::sleep(Duration::from_millis(16));
            }

            // Cleanup
            (*ximage).data = ptr::null_mut(); // Prevent double-free
            XDestroyImage(ximage);
            XFreeGC(display, gc);
            XDestroyWindow(display, window);
            if has_argb {
                XFreeColormap(display, colormap);
            }
            XCloseDisplay(display);

            Ok(())
        }
    }
}

// ============================================================================
// macOS Implementation
// ============================================================================
#[cfg(target_os = "macos")]
mod macos {
    use super::*;

    // A native transparent Cocoa splash proved unreliable to build across
    // toolchains (the AppKit bindings used here don't resolve on current
    // targets). On macOS we fall back to simply waiting for the configured
    // display duration; the application's own window appears normally right
    // afterwards, so the only thing lost is the pre-launch splash image.
    pub fn show_splash_window(
        _rgba: &image::RgbaImage,
        _width: u32,
        _height: u32,
        _x: i32,
        _y: i32,
        config: &SplashConfig,
    ) -> Result<(), String> {
        std::thread::sleep(config.display_duration);
        Ok(())
    }
}
