use enigo::*;

use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::IntoPyDict;

// Local module
mod circular_buffer;
mod hand_detector;

fn main() -> PyResult<()> {
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

        // Main Loop
        loop {
            let hand =
                hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

            // Use a circular buffer to filter high frequencies with median filter
            remanant_images.append(hand._wrist_pos);
            let hand_position: (i32, i32) = remanant_images.median_filter();

            println!("{:?}", hand._gesture);
            println!("{:?}", previous_hand._gesture);

            if hand._gesture != hand_detector::gesture::void {
                e.mouse_move_to(hand_position.0, hand_position.1);
            }

            if hand_detector::has_gesture_changed(hand, previous_hand) {
                match previous_hand._gesture {
                    hand_detector::gesture::thumb_index_pinched => e.mouse_up(MouseButton::Left),
                    hand_detector::gesture::thumb_middle_pinched => e.mouse_up(MouseButton::Right),
                    _ => (),
                }
                match hand._gesture {
                    hand_detector::gesture::thumb_index_pinched => e.mouse_down(MouseButton::Left),
                    hand_detector::gesture::thumb_middle_pinched => {
                        e.mouse_down(MouseButton::Right)
                    }
                    _ => (),
                }
            }

            previous_hand = hand;
        }
    })
}
