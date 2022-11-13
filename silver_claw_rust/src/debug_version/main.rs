#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use enigo::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

use std::ffi::c_void;
use std::ffi::CString;
use std::io;
use std::mem::zeroed;
use std::ptr::null;

use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;

// Local module
use silver_claw_lib::*;

fn main() -> PyResult<()> {
    // Create taskbar icon
    let hwnd: HWND;
    #[cfg(target_family = "windows")]
    {
        unsafe {
            // Create Class
            let mut wc: WNDCLASSA = zeroed();
            let class_name = CString::new("lpClassName").unwrap();
            let window_name = CString::new("lpWindowName").unwrap();

            wc.lpfnWndProc = Some(taskbar::window_proc);

            wc.hInstance = GetModuleHandleA(null());
            wc.lpszClassName = class_name.as_ptr() as *const i8;

            RegisterClassA(&wc);

            // Create Window
            hwnd = CreateWindowExA(
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
            taskbar::create(hwnd);
        }
    }

    #[cfg(target_family = "unix")]
    {}
    #[cfg(target_os = "macos")]
    {}

    // Ask for calibration mode.
    println!("Do you want to calibrate the system ? o/n");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut calibration: bool = false;

    if input.eq(&String::from('o')) {
        calibration = true;
    }

    // Init
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let locals = [(
            "hand_landmarks_detector",
            py.import("hand_landmarks_detector")?,
        )]
        .into_py_dict(py);

        let code = "hd = hand_landmarks_detector.hand_detector()";
        py_run!(py, *locals, code);
        let code_get_first_hand = "hd.get_landmarks(0)";
        let code_get_second_hand = "hd.get_landmarks(1)";

        let mut e = Enigo::new();

        let mut hands = (
            hand_detector::hand_state::new(),
            hand_detector::hand_state::new(),
        );

        let mut last_state: hand_detector::state = hand_detector::state::asleep;

        if calibration {}

        // Main Loop
        loop {
            let mut msg: MSG = unsafe { zeroed() };

            unsafe { PeekMessageA(&mut msg, hwnd, WM_MOUSEFIRST, WM_MOUSELAST, PM_REMOVE) };

            hands.0.compute_hand_state(
                py.eval(code_get_first_hand, None, Some(&locals))?
                    .extract()?,
            )?;

            hands.1.compute_hand_state(
                py.eval(code_get_second_hand, None, Some(&locals))?
                    .extract()?,
            )?;

            #[cfg(debug_assertions)]
            {
                println!("{:?}", hands.0._state);
                println!("{:?}", hands.1._state);
            }

            if hands.0._state == hand_detector::state::awake
                || hands.1._state == hand_detector::state::awake
            {
                last_state = hand_detector::state::awake;
            }

            if last_state != hand_detector::state::asleep {
                let new_position: (i32, i32) = match hands.0._wrist_pos {
                    Some(pos0) => match hands.1._wrist_pos {
                        Some(pos1) => ((pos0.0 + pos1.0) / 2, (pos0.1 + pos1.1) / 2),
                        None => pos0,
                    },
                    None => match hands.1._wrist_pos {
                        Some(pos1) => pos1,
                        None => (0i32, 0i32),
                    },
                };

                e.mouse_move_to(new_position.0, new_position.1);

                if hands.0._state == hand_detector::state::left_clicked
                    || hands.1._state == hand_detector::state::left_clicked
                {
                    if last_state == hand_detector::state::right_clicked {
                        e.mouse_up(MouseButton::Right);
                    }
                    e.mouse_down(MouseButton::Left);
                } else if hands.0._state == hand_detector::state::right_clicked
                    || hands.1._state == hand_detector::state::right_clicked
                {
                    if last_state == hand_detector::state::left_clicked {
                        e.mouse_up(MouseButton::Left);
                    }
                    e.mouse_down(MouseButton::Right);
                    last_state = hand_detector::state::right_clicked;
                } else {
                    if last_state == hand_detector::state::right_clicked {
                        e.mouse_up(MouseButton::Right);
                    } else if last_state == hand_detector::state::left_clicked {
                        e.mouse_up(MouseButton::Left);
                    }
                    if hands.0._state == hand_detector::state::asleep
                        || hands.1._state == hand_detector::state::asleep
                    {
                        last_state = hand_detector::state::asleep;
                    }
                }
            }
        }
    })
}

/*fn calibrate(py: Python, locals: &PyDict, code: &str) -> PyResult<()> {
    let mut previous_hand =
        hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

    let mut sleep: bool = true;

    loop {
        let hand = hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

        if sleep {
            if hand._gesture == hand_detector::gesture::open {
                sleep = false;
            }
        } else {
            match hand._gesture {
                hand_detector::gesture::void => {}
                hand_detector::gesture::closed => sleep = true,
                _ => {}
            }

            previous_hand = hand;
        }
    }
}*/
