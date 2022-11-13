#![allow(non_snake_case)]

extern crate winapi;

use std::ffi::OsStr;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{DefWindowProcA, PostQuitMessage, WM_RBUTTONDOWN};

const MAGIC_ID: UINT = 2209;

/**
 * Window procedure called each time the user interacts with the notification icon.
 */
pub unsafe extern "system" fn window_proc(
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
        if uMsg == MAGIC_ID {
            #[cfg(debug_assertions)]
            {
                // idk why thread ID is wrong but... it's fine
                println!("Magic ID");
            }

            PostQuitMessage(0);
        }
    }

    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

pub unsafe fn create(hwnd: HWND) -> winapi::um::shellapi::NOTIFYICONDATAW {
    let WM_MYMESSAGE = winapi::um::winuser::WM_APP + 100;
    let trayToolTip: String = "text".to_string();
    let mut trayToolTipInt: [u16; 128] = [0; 128];
    let trayToolTipOS: &OsStr = OsStr::new(&*trayToolTip);
    let trayToolTipUTF16: Vec<u16> = trayToolTipOS.encode_wide().collect::<Vec<u16>>();
    trayToolTipInt[..trayToolTipUTF16.len()].copy_from_slice(&trayToolTipUTF16);

    let iconAddress: String = "icon.ico".to_string();
    let iconAddressOS: &OsStr = OsStr::new(&*iconAddress);
    let iconAddressUTF16: Vec<u16> = iconAddressOS.encode_wide().collect::<Vec<u16>>();
    let icon: *const u16 = iconAddressUTF16.as_ptr();

    let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = zeroed();

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
    nid
}

pub unsafe fn delete(nid: &mut winapi::um::shellapi::NOTIFYICONDATAW) {
    winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, nid);
}
