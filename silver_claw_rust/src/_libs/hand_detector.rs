// An attribute to hide warnings for unused code/variables.
#![allow(dead_code)]
#![allow(unused_variables)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use winapi::um::winuser;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

mod calibration;
mod circular_buffer;
mod geometry;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum gesture {
    open,
    closed,
    none,

    thumb_index_pinched,
    thumb_middle_pinched,

    transition,
    void,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum state {
    asleep,
    awake,

    drag,
    left_clicked,
    right_clicked,

    not_detected,
}

#[derive(Debug, PartialEq, Clone)]
pub struct hand_state {
    pub _wrist_pos: Option<(i32, i32)>,
    pub _state: state,

    pub _buffer: circular_buffer::circular_buffer,
}

impl hand_state {
    pub fn new() -> Self {
        hand_state {
            _wrist_pos: None,
            _state: state::asleep,
            _buffer: circular_buffer::circular_buffer::default(),
        }
    }

    pub fn compute_hand_state(&mut self, landmarks: &PyAny) -> PyResult<()> {
        if landmarks.downcast::<PyList>().is_ok() {
            let landmarks: &PyList = landmarks.downcast()?;

            let landmarks_coordinates: Vec<(f32, f32, f32)> =
                landmarks.extract::<Vec<(f32, f32, f32)>>()?;

            let _gesture = compute_gesture(&landmarks_coordinates);

            match _gesture {
                gesture::open => {
                    if self._state == state::asleep {
                        self.compute_wrist_pos(&landmarks_coordinates);
                    }
                    self._state = state::awake;
                }
                gesture::closed => {
                    if self._state != state::asleep {
                        self._buffer.resize(3usize);
                    }
                    self._state = state::asleep;
                    self._wrist_pos = None;
                }
                gesture::thumb_index_pinched => {
                    let pos = compute_wrist_pos(&landmarks_coordinates);
                    self._buffer.append(pos);
                    self._buffer.reevaluate_size();

                    self._wrist_pos = Some(self._buffer.mean_filter());

                    self._state = state::drag;
                }
                gesture::thumb_middle_pinched => {
                    if landmarks_coordinates[1].0 < landmarks_coordinates[0].0 {
                        self._state = state::left_clicked;
                    } else {
                        self._state = state::right_clicked;
                    }
                }
                gesture::void => {
                    self._state = state::not_detected;
                }
                _ => {}
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn compute_wrist_pos(&mut self, landmarks_coordinates: &Vec<(f32, f32, f32)>) {
        self._wrist_pos = Some(compute_wrist_pos(landmarks_coordinates));
    }
}

/**
 * Screen dimensions singleton.
 *
 * /!\ Windows only for now.
 * TODO: Get screen info for MacOS and Ubuntu.
 */

struct screen_info {
    _dimensions: Option<(f32, f32)>,
}

static mut SCREEN_INFO: screen_info = screen_info { _dimensions: None };

/**
 * Returns the position where the mouse should be placed on the screen,
 * according to the hand coordinates.
 */
fn compute_wrist_pos(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> (i32, i32) {
    unsafe {
        match SCREEN_INFO._dimensions {
            Some(_a) => (),
            None => {
                #[cfg(target_family = "windows")]
                {
                    SCREEN_INFO._dimensions = Some((
                        winuser::GetSystemMetrics(winuser::SM_CXSCREEN) as f32,
                        winuser::GetSystemMetrics(winuser::SM_CYSCREEN) as f32,
                    ));
                }

                #[cfg(target_family = "unix")]
                {}
                #[cfg(target_os = "macos")]
                {}
            }
        }
    }

    let screen_width = unsafe { SCREEN_INFO._dimensions.unwrap().0 };
    let screen_height = unsafe { SCREEN_INFO._dimensions.unwrap().1 };

    let mut is_left_hand: i32 = 1;

    if landmarks_coordinates[1].0 > landmarks_coordinates[0].0 {
        is_left_hand = -1i32;
    }

    let mut res: (i32, i32) = calibration::CONFIG.with(|config| {
        (
            ((((screen_width - landmarks_coordinates[0].0 * screen_width)
                * config._calibration.x_speed_multiplicator)
                - (screen_width * config._calibration.x_offset_multiplicator)) as i32)
                + (is_left_hand * config._calibration.x_offset),
            (((landmarks_coordinates[0].1
                * screen_height
                * config._calibration.y_speed_multiplicator)
                - (screen_height * config._calibration.y_offset_multiplicator))
                as i32)
                - config._calibration.y_offset,
        )
    });

    if res.0 < 0i32 {
        res.0 = 0i32;
    } else if res.0 > screen_width as i32 {
        res.0 = screen_width as i32;
    }

    if res.1 < 0i32 {
        res.1 = 0i32;
    } else if res.1 > screen_height as i32 {
        res.1 = screen_height as i32;
    }

    res
}

const THUMB_INDEX_RATIO: f32 = 2.25f32;
const THUMB_MIDDLE_RATIO: f32 = 2f32;
const LEFT_CLIC_TRANSITION_RATIO: f32 = 2f32;
const RIGHT_CLIC_TRANSITION_RATIO: f32 = 1.75f32;

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

    if thumb_index_distance < (thumb_middle_distance / THUMB_INDEX_RATIO) {
        gesture::thumb_index_pinched
    } else if thumb_middle_distance < (thumb_index_distance / THUMB_MIDDLE_RATIO) {
        gesture::thumb_middle_pinched
    } else if thumb_index_distance < (thumb_middle_distance / LEFT_CLIC_TRANSITION_RATIO)
        || thumb_middle_distance < (thumb_index_distance / RIGHT_CLIC_TRANSITION_RATIO)
    {
        gesture::transition
    } else if compute_open_hand(landmarks_coordinates) {
        gesture::open
    } else if compute_closed_hand(landmarks_coordinates) {
        gesture::closed
    } else {
        gesture::none
    }
}

/**
 * We will consider an angle between these values to be ~= 0°.
 */
const ANGLE_LOW_MARGIN: f32 = -10f32;
const ANGLE_HIGH_MARGIN: f32 = 10f32;

fn compute_open_hand(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> bool {
    let mut r: bool = true;

    let mut n: usize = 6;
    loop {
        loop {
            let a: Vec<f32> = vec![
                landmarks_coordinates[n - 1].0,
                landmarks_coordinates[n - 1].1,
                landmarks_coordinates[n - 1].2,
            ];

            let b: Vec<f32> = vec![
                landmarks_coordinates[n].0,
                landmarks_coordinates[n].1,
                landmarks_coordinates[n].2,
            ];

            let c: Vec<f32> = vec![
                landmarks_coordinates[n + 1].0,
                landmarks_coordinates[n + 1].1,
                landmarks_coordinates[n + 1].2,
            ];

            let v: Vec<f32> = geometry::compute_vec_from_points(&a, &b);
            let w: Vec<f32> = geometry::compute_vec_from_points(&b, &c);

            let angle: f32 = geometry::compute_angle(&v, &w);
            if !(ANGLE_LOW_MARGIN < angle && angle < ANGLE_HIGH_MARGIN) {
                r = false;
            }

            n += 1;
            if n % 2 == 0 {
                break;
            }
        }
        n += 2;
        if n > 19 {
            break;
        }
    }

    r
}

fn compute_closed_hand(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> bool {
    let mut r: bool = true;

    let thumb_magnetude: f32 = compute_thumb_magnetude(landmarks_coordinates);

    // For each finger except thumb, if magnetude is lower than thumb magnetude, the hand is considered closed.
    for i in (5usize..=17usize).step_by(2usize) {
        let a: Vec<f32> = vec![
            landmarks_coordinates[i].0,
            landmarks_coordinates[i].1,
            landmarks_coordinates[i].2,
        ];

        let b: Vec<f32> = vec![
            landmarks_coordinates[i + 3].0,
            landmarks_coordinates[i + 3].1,
            landmarks_coordinates[i + 3].2,
        ];

        let v: Vec<f32> = geometry::compute_vec_from_points(&a, &b);
        let m: f32 = geometry::magnitude(&v);

        if m > thumb_magnetude {
            r = false;
        }
    }

    r
}

fn compute_thumb_magnetude(landmarks_coordinates: &Vec<(f32, f32, f32)>) -> f32 {
    let a: Vec<f32> = vec![
        landmarks_coordinates[2].0,
        landmarks_coordinates[2].1,
        landmarks_coordinates[2].2,
    ];

    let b: Vec<f32> = vec![
        landmarks_coordinates[4].0,
        landmarks_coordinates[4].1,
        landmarks_coordinates[4].2,
    ];

    let v: Vec<f32> = geometry::compute_vec_from_points(&a, &b);
    geometry::magnitude(&v)
}
