#![allow(non_snake_case)]

extern crate winapi;

use std::ffi::c_void;
use std::ffi::{CString, OsStr};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HMENU__, HWND, HWND__};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    CreateWindowExA, DefWindowProcA, PostQuitMessage, RegisterClassA, ShowWindow, WM_RBUTTONDOWN,
    WNDCLASSA,
};

const MAGIC_ID: UINT = 2209;
pub static mut EXIT: bool = false;

/**
 * Window procedure called each time the user interacts with the notification icon.
 */
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    #[cfg(debug_assertions)]
    {
        println!("Entered window proc");
    }

    if (lParam as UINT) == WM_RBUTTONDOWN {
        println!("Right clic on icon");
        EXIT = true;
    }

    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

pub unsafe fn create() -> HWND {
    // Create Class
    let mut wc: WNDCLASSA = zeroed();
    let class_name = CString::new("lpClassName").unwrap();
    let window_name = CString::new("lpWindowName").unwrap();

    wc.lpfnWndProc = Some(window_proc);

    wc.hInstance = GetModuleHandleA(null());
    wc.lpszClassName = class_name.as_ptr() as *const i8;

    RegisterClassA(&wc);

    // Create Window
    let hwnd = CreateWindowExA(
        0,
        class_name.as_ptr() as *const i8,
        window_name.as_ptr() as *const i8,
        winapi::um::winuser::WS_OVERLAPPEDWINDOW,
        winapi::um::winuser::CW_USEDEFAULT,
        winapi::um::winuser::CW_USEDEFAULT,
        winapi::um::winuser::CW_USEDEFAULT,
        winapi::um::winuser::CW_USEDEFAULT,
        null::<*mut HWND__>() as *mut HWND__,
        null::<*mut HMENU__>() as *mut HMENU__,
        wc.hInstance,
        null::<*mut c_void>() as *mut c_void,
    );

    // Show Window
    ShowWindow(hwnd, 0);

    // Create Taskbar
    let WM_MYMESSAGE = winapi::um::winuser::WM_APP + 100;
    let trayToolTip: String = "Silver Claw Mouse Driver".to_string();
    let mut trayToolTipInt: [u16; 128] = [0; 128];
    let trayToolTipOS: &OsStr = OsStr::new(&*trayToolTip);
    let trayToolTipUTF16: Vec<u16> = trayToolTipOS.encode_wide().collect::<Vec<u16>>();
    trayToolTipInt[..trayToolTipUTF16.len()].copy_from_slice(&trayToolTipUTF16);

    let iconAddress: String = "icon.ico".to_string();
    let iconAddressOS: &OsStr = OsStr::new(&*iconAddress);
    let iconAddressUTF16: Vec<u16> = iconAddressOS.encode_wide().collect::<Vec<u16>>();
    let icon: *const u16 = iconAddressUTF16.as_ptr();

    let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = zeroed();

    #[cfg(debug_assertions)]
    {
        println!(
            "icon: {:?}",
            winapi::um::winuser::LoadIconW(
                winapi::um::libloaderapi::GetModuleHandleW(null_mut()),
                icon
            )
        );
    }

    nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = MAGIC_ID;
    nid.uCallbackMessage = WM_MYMESSAGE;
    nid.hIcon = winapi::um::winuser::LoadIconW(null_mut(), icon);
    nid.szTip = trayToolTipInt;
    nid.uFlags = winapi::um::shellapi::NIF_MESSAGE
        | winapi::um::shellapi::NIF_ICON
        | winapi::um::shellapi::NIF_TIP;

    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid);
    hwnd
}

pub unsafe fn delete(nid: &mut winapi::um::shellapi::NOTIFYICONDATAW) {
    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, nid);
}
