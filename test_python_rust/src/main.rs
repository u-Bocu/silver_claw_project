use pyo3::prelude::*;

fn main() -> PyResult<()> {

    pyo3::prepare_freethreaded_python();
    let py_hand_detector = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../python_hand_detection/hand_landmarks_detector.py"));
    let py_main = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../python_hand_detection/main.py"));

    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from_code(py, py_hand_detector, "hand_landmarks_detector", "hand_landmarks_detector")?;

        let app: Py<PyAny> = PyModule::from_code(py, py_main, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from_python?);
    Ok(())
}
