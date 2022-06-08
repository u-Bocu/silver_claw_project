// An attribute to hide warnings for unused code/variables.
#![allow(dead_code)]
#![allow(unused_variables)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

const SCREEN_HEIGHT: f32 = 768f32;
const SCREEN_WIDTH: f32 = 1366f32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum gesture {
    open,
    closed,
    none,
    thumb_index_pinched,
    thumb_middle_pinched,
    void,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct hand_state {
    pub _thumb_pos: (i32, i32),
    pub _gesture: gesture,
}

pub fn get_hand_state(landmarks: &PyAny) -> PyResult<hand_state> {
    if landmarks.downcast::<PyList>().is_ok() {
        let landmarks: &PyList = landmarks.downcast()?;

        let landmarks_coordinates: Vec<(f32, f32, f32)> =
            landmarks.extract::<Vec<(f32, f32, f32)>>()?;

        //println!("{}", landmarks);

        Ok(hand_state {
            _thumb_pos: compute_thumb_pos(&landmarks_coordinates),
            _gesture: compute_gesture(&landmarks_coordinates),
        })
    } else {
        Ok(hand_state {
            _thumb_pos: (0i32, 0i32),
            _gesture: gesture::void,
        })
    }
}

/**
 * Does exactly what you think it does.
 * Returns true is the gesture has changed between h0 and h1.
 */
#[inline(always)]
pub fn has_gesture_changed(h0: hand_state, h1: hand_state) -> PyResult<bool> {
    if h0._gesture == h1._gesture {
        Ok(false)
    } else {
        Ok(true)
    }
}

const TRUNCATURE_SIZE: i32 = 5i32;

/**
 * Returns the position where the mouse should be placed on the screen,
 * according to the thumb position on the image.
 */
fn compute_thumb_pos(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> (i32, i32) {
    // Truncate thumb position to filter white noise.
    let c: (f32, f32) = (
        (landmarks_coordinates[4].0 / 2f32.powi(TRUNCATURE_SIZE)) * 2f32.powi(TRUNCATURE_SIZE),
        (landmarks_coordinates[4].1 / 2f32.powi(TRUNCATURE_SIZE)) * 2f32.powi(TRUNCATURE_SIZE),
    );

    let res: (i32, i32) = (
        (SCREEN_WIDTH - c.0 * SCREEN_WIDTH) as i32,
        (c.1 * SCREEN_HEIGHT) as i32,
    );
    res
}

const FINGER_DISTANCE_RATIO: f32 = 3f32;

/**
 * Returns the hand gesture recognized with geometry.
 * Geometry is simpler and quicker than ML in our case.
 */
fn compute_gesture(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> gesture {
    let thumb_index_distance: f32 = f32::sqrt(
        (landmarks_coordinates[4].0 - landmarks_coordinates[8].0).powi(2i32)
            + (landmarks_coordinates[4].1 - landmarks_coordinates[8].1).powi(2i32)
            + (landmarks_coordinates[4].2 - landmarks_coordinates[8].2).powi(2i32),
    );

    let thumb_middle_distance: f32 = f32::sqrt(
        (landmarks_coordinates[4].0 - landmarks_coordinates[12].0).powi(2i32)
            + (landmarks_coordinates[4].1 - landmarks_coordinates[12].1).powi(2i32)
            + (landmarks_coordinates[4].2 - landmarks_coordinates[12].2).powi(2i32),
    );

    let index_middle_distance: f32 = f32::sqrt(
        (landmarks_coordinates[8].0 - landmarks_coordinates[12].0).powi(2i32)
            + (landmarks_coordinates[8].1 - landmarks_coordinates[12].1).powi(2i32)
            + (landmarks_coordinates[8].2 - landmarks_coordinates[12].2).powi(2i32),
    );

    if thumb_index_distance < (thumb_middle_distance / FINGER_DISTANCE_RATIO) {
        gesture::thumb_index_pinched
    } else if thumb_middle_distance < (thumb_index_distance / FINGER_DISTANCE_RATIO) {
        gesture::thumb_middle_pinched
    } else {
        gesture::none
    }
}
