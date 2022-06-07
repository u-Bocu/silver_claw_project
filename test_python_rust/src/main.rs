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

        let remanant_images: circular_buffer::circular_buffer =
            circular_buffer::circular_buffer::default();

        // Main Loop
        loop {
            let hand =
                hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?)?;

            match hand._gesture {
                hand_detector::gesture::open => (),
                hand_detector::gesture::closed => (),
                hand_detector::gesture::none => (),
                hand_detector::gesture::thumb_index_pinched => (),
                hand_detector::gesture::thumb_middle_pinched => (),
            }
        }
    })
}
