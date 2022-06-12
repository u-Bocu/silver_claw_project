// An attribute to hide warnings for unused code/variables.
#![allow(dead_code)]
#![allow(unused_variables)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use winapi::um::winuser;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

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
    pub _wrist_pos: (i32, i32),
    pub _gesture: gesture,
}

pub fn get_hand_state(landmarks: &PyAny) -> PyResult<hand_state> {
    if landmarks.downcast::<PyList>().is_ok() {
        let landmarks: &PyList = landmarks.downcast()?;

        let landmarks_coordinates: Vec<(f32, f32, f32)> =
            landmarks.extract::<Vec<(f32, f32, f32)>>()?;

        //println!("{}", landmarks);

        Ok(hand_state {
            _wrist_pos: compute_wrist_pos(&landmarks_coordinates),
            _gesture: compute_gesture(&landmarks_coordinates),
        })
    } else {
        Ok(hand_state {
            _wrist_pos: (0i32, 0i32),
            _gesture: gesture::void,
        })
    }
}

/**
 * Does exactly what you think it does.
 * Returns true is the gesture has changed between h0 and h1.
 */
#[inline(always)]
pub fn has_gesture_changed(h0: hand_state, h1: hand_state) -> bool {
    if h0._gesture == h1._gesture || h0._gesture == gesture::void || h1._gesture == gesture::void {
        false
    } else {
        true
    }
}

const TRUNCATURE_SIZE: i32 = 10i32;
const X_SPEED_MULTIPLICATOR: f32 = 1f32;
const Y_SPEED_MULTIPLICATOR: f32 = 1f32;

/**
 * Screen dimensions singleton.
 *
 * /!\ Windows only for now
 */

struct screen_info {
    _dimensions: Option<(f32, f32)>,
}

static mut SCREEN_INFO: screen_info = screen_info { _dimensions: None };

/**
 * Returns the position where the mouse should be placed on the screen,
 * according to the thumb position on the image.
 */
fn compute_wrist_pos(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> (i32, i32) {
    unsafe {
        match SCREEN_INFO._dimensions {
            Some(_a) => (),
            None => {
                SCREEN_INFO._dimensions = Some((
                    winuser::GetSystemMetrics(winuser::SM_CXSCREEN) as f32,
                    winuser::GetSystemMetrics(winuser::SM_CYSCREEN) as f32,
                ));
            }
        }
    }

    let screen_width = unsafe { SCREEN_INFO._dimensions.unwrap().0 };
    let screen_height = unsafe { SCREEN_INFO._dimensions.unwrap().1 };
    // Truncate thumb position to filter white noise.
    let c: (f32, f32) = (
        (landmarks_coordinates[0].0 / 2f32.powi(TRUNCATURE_SIZE)) * 2f32.powi(TRUNCATURE_SIZE),
        (landmarks_coordinates[0].1 / 2f32.powi(TRUNCATURE_SIZE)) * 2f32.powi(TRUNCATURE_SIZE),
    );

    let res: (i32, i32) = (
        ((screen_width - c.0 * screen_width) / X_SPEED_MULTIPLICATOR) as i32,
        (c.1 * screen_height / Y_SPEED_MULTIPLICATOR) as i32,
    );
    res
}

const LEFT_CLIC_RATIO: f32 = 2f32;
const RIGHT_CLIC_RATIO: f32 = 2f32;

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

    if thumb_index_distance < (thumb_middle_distance / LEFT_CLIC_RATIO) {
        gesture::thumb_index_pinched
    } else if thumb_middle_distance < (thumb_index_distance / RIGHT_CLIC_RATIO) {
        gesture::thumb_middle_pinched
    } else {
        gesture::none
    }
}
