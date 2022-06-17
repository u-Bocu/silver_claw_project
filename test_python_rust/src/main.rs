use enigo::*;

use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;

// Local module
mod circular_buffer;
mod hand_detector;
mod taskbar;

fn main() -> PyResult<()> {
    // Create taskbar icon
    let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = taskbar::create();
    taskbar::delete(&mut nid);

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
        let code = "hd.get_landmarks()";

        let mut e = Enigo::new();

        let mut remanant_images: circular_buffer::circular_buffer =
            circular_buffer::circular_buffer::default();

        let mut previous_hand =
            hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

        let mut sleep: bool = true;

        // Main Loop
        loop {
            let hand =
                hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

            println!("{:?}", hand._gesture);

            if sleep {
                if hand._gesture == hand_detector::gesture::open {
                    sleep = false;
                }
            } else {
                // Use a circular buffer to filter high frequencies with median filter
                match hand._wrist_pos {
                    Some(pos) => {
                        remanant_images.append(pos);
                    }
                    None => (),
                }

                let hand_position: (i32, i32) = remanant_images.median_filter();

                if hand._gesture != hand_detector::gesture::void
                    && hand._gesture != hand_detector::gesture::thumb_middle_pinched
                {
                    e.mouse_move_to(hand_position.0, hand_position.1);
                }

                if hand._gesture != hand_detector::gesture::transition {
                    if hand_detector::has_gesture_changed(hand, previous_hand) {
                        match previous_hand._gesture {
                            hand_detector::gesture::thumb_index_pinched => {
                                e.mouse_up(MouseButton::Left)
                            }
                            hand_detector::gesture::thumb_middle_pinched => {
                                e.mouse_up(MouseButton::Right)
                            }
                            _ => (),
                        }
                        match hand._gesture {
                            hand_detector::gesture::thumb_index_pinched => {
                                e.mouse_down(MouseButton::Left);
                                remanant_images = circular_buffer::circular_buffer::new(16usize);
                            }
                            hand_detector::gesture::thumb_middle_pinched => {
                                e.mouse_down(MouseButton::Right);
                                remanant_images = circular_buffer::circular_buffer::new(16usize);
                            }
                            hand_detector::gesture::closed => sleep = true,
                            _ => remanant_images = circular_buffer::circular_buffer::new(4usize),
                        }
                    }

                    previous_hand = hand;
                }
            }
        }
    })
}
