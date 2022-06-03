use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::py_run;

// Local module
mod hand_detector;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let locals = [("hand_landmarks_detector", py.import("hand_landmarks_detector")?)].into_py_dict(py);
        let code = "hd = hand_landmarks_detector.hand_detector()";
        py_run!(py, *locals, code);

        let code = "hd.get_landmarks()";

        let res: &hand_detector::hand_state = &hand_detector::hand_state {
            _thumb_pos: (0.0, 0.0),
            _gesture: hand_detector::gesture::none,
        };
        
        loop {
            hand_detector::get_hand_state(py.eval(code, None, Some(&locals))?.extract()?, res)?;
        }
    })
}

/*fn foo(_a: &PyAny) -> &PyAny {
    // Do whatever calculus I need to do.
    _a
}*/

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || thread_main);

//     let x: Result<i16, mpsc::RecvError> = rx.recv();
// }

// fn thread_main(tx: mpsc::Sender<i16>) {
//     pyo3::prepare_freethreaded_python();
//     Python::with_gil(|py| {

//         let locals = [("hand_landmarks_detector", py.import("hand_landmarks_detector")?)].into_py_dict(py);
//         let code = "hand_landmarks_detector.start()";
        
//         loop {
//             tx.send(45).unwrap();
//             let res: &PyAny = py.eval(code, None, Some(&locals))?.extract()?;
//             println!("{:?}", res);
//         }
//     })
// }

//  Working Python hand_detection module import + execute
// fn main() -> PyResult<()> {
//     pyo3::prepare_freethreaded_python();
//     Python::with_gil(|py| {

//         let locals = [("hand_landmarks_detector", py.import("hand_landmarks_detector")?)].into_py_dict(py);
//         let code = "hand_landmarks_detector.start()";

//         loop {
//             let res: &PyAny = py.eval(code, None, Some(&locals))?.extract()?;
//             println!("{:?}", res);
//         }
//     })
// }

//   Working Python os module import + execute
// fn main() -> PyResult<()> {
//     pyo3::prepare_freethreaded_python();
//     Python::with_gil(|py| {
//         let sys = py.import("sys")?;
//         let version: String = sys.getattr("version")?.extract()?;

//         let locals = [("os", py.import("os")?)].into_py_dict(py);
//         let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
//         let user: String = py.eval(code, None, Some(&locals))?.extract()?;

//         println!("Hello {}, I'm Python {}", user, version);
//         Ok(())
//     })
// }
