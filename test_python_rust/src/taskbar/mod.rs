#![allow(non_snake_case)]

extern crate winapi;

use std::ffi::OsStr;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

pub fn create() -> winapi::um::shellapi::NOTIFYICONDATAW {
    let hWnd = winapi::um::wincon::GetConsoleWindow;

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

    let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe { zeroed() };

    unsafe {
        nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32;
        nid.hWnd = hWnd();
        nid.uID = 1001;
        nid.uCallbackMessage = WM_MYMESSAGE;
        nid.hIcon = winapi::um::winuser::LoadIconW(null_mut(), icon);
        nid.szTip = trayToolTipInt;
        nid.uFlags = winapi::um::shellapi::NIF_MESSAGE
            | winapi::um::shellapi::NIF_ICON
            | winapi::um::shellapi::NIF_TIP;
    };

    unsafe { winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid) };
    nid
}

pub fn delete(nid: &mut winapi::um::shellapi::NOTIFYICONDATAW) {
    unsafe { winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, nid) };
}
