// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use pyo3::prelude::*;
use pyo3::types::{
    PyAny,
    PyList,
};

pub enum gesture {
    open,
    closed,
    none,
}
    
pub struct hand_state {
    pub _thumb_pos: (f32, f32),
    pub _gesture: gesture,
}

pub fn get_hand_state(landmarks: &PyAny, mut _hand_state: &hand_state) -> PyResult<()> {
    if landmarks.downcast::<PyList>().is_ok() {

        let landmarks: &PyList = landmarks.downcast()?;

        let landmarks_coordinates: Vec<(f32, f32, f32)> = landmarks.extract::<Vec<(f32, f32, f32)>>()?;

        //println!("{:?}", landmarks_coordinates);

        let c: (f32, f32) = (landmarks_coordinates[4].0, landmarks_coordinates[4].1);

        _hand_state = &hand_state {
            _thumb_pos: &c,
            _gesture: gesture::none,
        };
    }

    Ok(())
}