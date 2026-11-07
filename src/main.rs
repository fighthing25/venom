#![windows_subsystem = "windows"]

mod addons;
mod app;
mod audio;
mod calendar;
mod capture;
mod clipboard;
mod config;
mod lyrics;
mod render;
mod sambung;
mod shelf;

use app::VenomIslandApp;
use std::sync::{Arc, Mutex};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, POINT, SIZE, WPARAM};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, SelectObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, BLENDFUNCTION, DIB_RGB_COLORS, AC_SRC_ALPHA, AC_SRC_OVER,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetCursorPos, GetMessageW,
    GetSystemMetrics, PostQuitMessage, RegisterClassExW, SetTimer, ShowWindow,
    TranslateMessage, UpdateLayeredWindow, HTCLIENT, HTTRANSPARENT, MSG, SM_CXSCREEN,
    SW_SHOW, ULW_ALPHA, WM_DESTROY, WM_LBUTTONDOWN, WM_MOUSEWHEEL,
    WM_NCHITTEST, WM_TIMER, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_POPUP,
};

static mut GLOBAL_APP: Option<Arc<Mutex<VenomIslandApp>>> = None;

fn main() {
    let app = Arc::new(Mutex::new(VenomIslandApp::new()));
    unsafe { GLOBAL_APP = Some(app.clone()); }

    let h_instance = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(None).unwrap() };
    let class_name = w!("VenomIslandWindowClass");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: windows::Win32::UI::WindowsAndMessaging::CS_HREDRAW | windows::Win32::UI::WindowsAndMessaging::CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: h_instance.into(),
        hIcon: windows::Win32::UI::WindowsAndMessaging::HICON(std::ptr::null_mut()),
        hCursor: unsafe { windows::Win32::UI::WindowsAndMessaging::LoadCursorW(None, windows::Win32::UI::WindowsAndMessaging::IDC_ARROW).unwrap() },
        hbrBackground: windows::Win32::Graphics::Gdi::HBRUSH(std::ptr::null_mut()),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: class_name,
        hIconSm: windows::Win32::UI::WindowsAndMessaging::HICON(std::ptr::null_mut()),
    };

    unsafe { RegisterClassExW(&wc); }

    let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let win_w = screen_w;
    let win_h = 440;

    let hwnd = unsafe {
        CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW,
            class_name,
            w!("VenomIsland"),
            WS_POPUP,
            0,
            0,
            win_w,
            win_h,
            HWND(std::ptr::null_mut()),
            windows::Win32::UI::WindowsAndMessaging::HMENU(std::ptr::null_mut()),
            h_instance,
            None,
        )
    };

    if hwnd.is_err() { return; }
    let hwnd = hwnd.unwrap();

    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOW);
        SetTimer(hwnd, 1, 16, None);
    }

    let mut msg = MSG::default();
    unsafe {
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_TIMER => {
            unsafe {
                if let Some(app_arc) = &GLOBAL_APP {
                    if let Ok(mut app) = app_arc.lock() {
                        let mut cursor_pt = POINT { x: 0, y: 0 };
                        let _ = GetCursorPos(&mut cursor_pt);
                        let screen_w = GetSystemMetrics(SM_CXSCREEN) as f32;

                        app.cursor_x = cursor_pt.x as f32;
                        app.cursor_y = cursor_pt.y as f32;

                        let cx = screen_w / 2.0;
                        let half_w = app.width_spring.current / 2.0;
                        let is_near_x = app.cursor_x >= (cx - half_w - 40.0) && app.cursor_x <= (cx + half_w + 40.0);
                        let is_near_y = app.cursor_y >= 0.0 && app.cursor_y <= (app.height_spring.current + 60.0);
                        app.is_hovered = is_near_x && is_near_y;

                        app.update(0.016);
                        render_layered_window(hwnd, &mut app, screen_w as u32, 440);
                    }
                }
            }
            LRESULT(0)
        }
        WM_NCHITTEST => {
            unsafe {
                let x = (lparam.0 & 0xFFFF) as i16 as f32;
                let y = ((lparam.0 >> 16) & 0xFFFF) as i16 as f32;

                if let Some(app_arc) = &GLOBAL_APP {
                    if let Ok(app) = app_arc.lock() {
                        let screen_w = GetSystemMetrics(SM_CXSCREEN) as f32;
                        let cx = screen_w / 2.0;
                        let cur_w = app.width_spring.current;
                        let cur_h = app.height_spring.current;

                        let inside_x = x >= (cx - (cur_w / 2.0)) && x <= (cx + (cur_w / 2.0));
                        let inside_y = y >= 12.0 && y <= (12.0 + cur_h);

                        if inside_x && inside_y {
                            return LRESULT(HTCLIENT as isize);
                        }
                    }
                }
                LRESULT(HTTRANSPARENT as isize)
            }
        }
        WM_LBUTTONDOWN => {
            unsafe {
                let mut pt = POINT { x: 0, y: 0 };
                let _ = GetCursorPos(&mut pt);
                let screen_w = GetSystemMetrics(SM_CXSCREEN) as f32;

                if let Some(app_arc) = &GLOBAL_APP {
                    if let Ok(mut app) = app_arc.lock() {
                        app.handle_click(pt.x as f32, pt.y as f32, screen_w);
                    }
                }
            }
            LRESULT(0)
        }
        WM_MOUSEWHEEL => {
            let delta = ((wparam.0 >> 16) & 0xFFFF) as i16;
            let step = if delta > 0 { 0.05 } else { -0.05 };
            audio::AudioManager::step_master_volume(step);
            LRESULT(0)
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0); }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn render_layered_window(hwnd: HWND, app: &mut VenomIslandApp, width: u32, height: u32) {
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    app.render(&mut pixmap.as_mut(), width as f32);

    let hdc_screen = windows::Win32::Graphics::Gdi::GetDC(HWND(std::ptr::null_mut()));
    let hdc_mem = CreateCompatibleDC(hdc_screen);

    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }],
    };

    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let hbitmap = CreateDIBSection(hdc_screen, &bmi, DIB_RGB_COLORS, &mut bits, None, 0).unwrap();
    let old_bitmap = SelectObject(hdc_mem, hbitmap);

    let src = pixmap.data();
    let dest = std::slice::from_raw_parts_mut(bits as *mut u8, (width * height * 4) as usize);

    for i in (0..src.len()).step_by(4) {
        let r = src[i];
        let g = src[i + 1];
        let b = src[i + 2];
        let a = src[i + 3];
        dest[i] = b;
        dest[i + 1] = g;
        dest[i + 2] = r;
        dest[i + 3] = a;
    }

    let pt_dst = POINT { x: 0, y: 0 };
    let size = SIZE { cx: width as i32, cy: height as i32 };
    let pt_src = POINT { x: 0, y: 0 };
    let blend = BLENDFUNCTION {
        BlendOp: AC_SRC_OVER as u8,
        BlendFlags: 0,
        SourceConstantAlpha: 255,
        AlphaFormat: AC_SRC_ALPHA as u8,
    };

    let _ = UpdateLayeredWindow(
        hwnd,
        hdc_screen,
        Some(&pt_dst),
        Some(&size),
        hdc_mem,
        Some(&pt_src),
        COLORREF(0),
        Some(&blend),
        ULW_ALPHA,
    );

    SelectObject(hdc_mem, old_bitmap);
    let _ = DeleteObject(hbitmap);
    let _ = DeleteDC(hdc_mem);
    let _ = windows::Win32::Graphics::Gdi::ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
}