#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use enigo::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::mem::zeroed;

use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;

// Local module
use silver_claw_lib::hand_detector::calibration;
use silver_claw_lib::*;

fn main() -> PyResult<()> {
    // Create taskbar icon
    let hwnd: HWND;
    #[cfg(target_family = "windows")]
    {
        hwnd = unsafe { taskbar::create() }
    }

    #[cfg(target_family = "unix")]
    {}
    #[cfg(target_os = "macos")]
    {}

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

        // Main Loop
        loop {
            if taskbar::EXIT.with(|exit| *exit == true) {
                break;
            }
            let mut msg: MSG = unsafe { zeroed() };

            #[cfg(target_family = "windows")]
            unsafe {
                PeekMessageA(&mut msg, hwnd, WM_RBUTTONDOWN, WM_RBUTTONDOWN, PM_REMOVE)
            };

            hands.0.compute_hand_state(
                py.eval(code_get_first_hand, None, Some(&locals))?
                    .extract()?,
            )?;

            // TODO: Optimize 2-hand mode (Python is so slow...)
            if calibration::CONFIG.with(|config| config.mode.get_mouse_mode())
                == hand_detector::calibration::mouse_mode::absolute
            {
                hands.1.compute_hand_state(
                    py.eval(code_get_second_hand, None, Some(&locals))?
                        .extract()?,
                )?;
            }

            #[cfg(debug_assertions)]
            {
                println!("Hand 0:{:?}", hands.0._state);
                println!("Hand 1:{:?}", hands.1._state);
                println!("Last state:{:?}", last_state);
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

                if calibration::CONFIG.with(|config| config.mode.get_mouse_mode())
                    == hand_detector::calibration::mouse_mode::absolute
                {
                    e.mouse_move_to(new_position.0, new_position.1);
                } else {
                    let new_position: (i32, i32) = match hands.0._shift {
                        Some(pos) => (pos.0, pos.1),
                        None => (0i32, 0i32),
                    };

                    e.mouse_move_relative(new_position.0, new_position.1);
                }

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
                        && hands.1._state == hand_detector::state::asleep
                    {
                        last_state = hand_detector::state::asleep;
                    }
                }
            }
        }

        calibration::CONFIG.with(|config| {
            config.save_calibration();
        });

        Ok(())
    })
}
