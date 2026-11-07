use windows::Win32::Foundation::{HWND, POINT};
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
    GetDIBits, GetPixel, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
    DIB_RGB_COLORS, SRCCOPY,
};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub struct ScreenCapture;

impl ScreenCapture {
    pub fn get_cursor_color() -> (u8, u8, u8, String) {
        unsafe {
            let mut pt = POINT { x: 0, y: 0 };
            let _ = GetCursorPos(&mut pt);
            let hdc_screen = GetDC(HWND(std::ptr::null_mut()));
            let color = GetPixel(hdc_screen, pt.x, pt.y);
            let _ = ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);

            let r = (color.0 & 0xFF) as u8;
            let g = ((color.0 >> 8) & 0xFF) as u8;
            let b = ((color.0 >> 16) & 0xFF) as u8;
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            (r, g, b, hex)
        }
    }

    pub fn capture_fullscreen_to_bgra() -> Option<(u32, u32, Vec<u8>)> {
        unsafe {
            let w = GetSystemMetrics(SM_CXSCREEN) as u32;
            let h = GetSystemMetrics(SM_CYSCREEN) as u32;
            let hdc_screen = GetDC(HWND(std::ptr::null_mut()));
            let hdc_mem = CreateCompatibleDC(hdc_screen);
            let hbm = CreateCompatibleBitmap(hdc_screen, w as i32, h as i32);
            let old_bm = SelectObject(hdc_mem, hbm);

            let _ = BitBlt(hdc_mem, 0, 0, w as i32, h as i32, hdc_screen, 0, 0, SRCCOPY);

            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w as i32,
                    biHeight: -(h as i32),
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

            let mut buffer: Vec<u8> = vec![0u8; (w * h * 4) as usize];
            GetDIBits(hdc_mem, hbm, 0, h, Some(buffer.as_mut_ptr() as *mut _), &mut bmi, DIB_RGB_COLORS);

            SelectObject(hdc_mem, old_bm);
            let _ = DeleteObject(hbm);
            let _ = DeleteDC(hdc_mem);
            let _ = ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);

            Some((w, h, buffer))
        }
    }
}