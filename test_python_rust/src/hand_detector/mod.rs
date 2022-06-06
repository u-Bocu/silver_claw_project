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
    thumb_index_pinched,
    thumb_middle_pinched,
}
    
pub struct hand_state {
    pub _thumb_pos: (f32, f32),
    pub _gesture: gesture,
}

pub fn get_hand_state(landmarks: &PyAny, mut _hand_state: &mut hand_state) -> PyResult<()> {
    if landmarks.downcast::<PyList>().is_ok() {

        let landmarks: &PyList = landmarks.downcast()?;

        let landmarks_coordinates: Vec<(f32, f32, f32)> = landmarks.extract::<Vec<(f32, f32, f32)>>()?;

        //println!("{:?}", landmarks_coordinates);
        _hand_state._thumb_pos = compute_thumb_pos(&landmarks_coordinates);
        _hand_state._gesture = compute_gesture(&landmarks_coordinates);
    }

    Ok(())
}

fn compute_thumb_pos(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> (f32, f32) {
    (landmarks_coordinates[4].0, landmarks_coordinates[4].1)
}

fn compute_gesture(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> gesture {
    gesture::none
}