#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use serde::{Deserialize, Serialize};
use winapi::um::winuser;

thread_local!( pub static CONFIG: config = config::new());
thread_local!( pub(crate) static SCREEN_INFO: screen_info = screen_info::default());

/**
 * Screen dimensions singleton.
 */

pub struct screen_info {
    pub _dimensions: (f32, f32),
}

impl Default for screen_info {
    fn default() -> Self {
        #[cfg(target_family = "windows")]
        {
            screen_info {
                _dimensions: unsafe {
                    (
                        winuser::GetSystemMetrics(winuser::SM_CXSCREEN) as f32,
                        winuser::GetSystemMetrics(winuser::SM_CYSCREEN) as f32,
                    )
                },
            }
        }
    }
}

/**
 * Configuration singleton.
 */

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct calibration {
    pub x_offset: i32,
    pub y_offset: i32,

    pub x_offset_multiplicator: f32,
    pub y_offset_multiplicator: f32,

    pub x_speed_multiplicator: f32,
    pub y_speed_multiplicator: f32,
}

#[derive(Debug, PartialEq)]
pub enum mouse_mode {
    absolute,
    relative,
}

#[derive(Debug, PartialEq)]
pub enum main_hand {
    left,
    right,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct mode {
    _absolute: bool,
    _relative: bool,

    _left_handed: bool,
    _right_handed: bool,

    pub _gui_on: bool,
}

impl mode {
    fn set_mouse_mode(&mut self, m: mouse_mode) {
        if m == mouse_mode::absolute {
            self._absolute = true;
            self._relative = false;
        } else {
            self._absolute = false;
            self._relative = true;
        }
    }

    pub fn get_mouse_mode(&self) -> mouse_mode {
        if self._absolute {
            mouse_mode::absolute
        } else {
            mouse_mode::relative
        }
    }

    fn set_main_hand(&mut self, h: main_hand) {
        if h == main_hand::left {
            self._left_handed = true;
            self._right_handed = false;
        } else {
            self._left_handed = false;
            self._right_handed = true;
        }
    }

    pub fn get_main_hand(&self) -> main_hand {
        if self._left_handed {
            main_hand::left
        } else {
            main_hand::right
        }
    }
}

impl Default for mode {
    fn default() -> Self {
        mode {
            _absolute: true,
            _relative: false,

            _left_handed: false,
            _right_handed: true,

            _gui_on: true,
        }
    }
}

const CONFIG_PATH: &str = "config.yaml";

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct config {
    pub _calibration: calibration,
    pub _mode: mode,
}

impl config {
    pub fn new() -> Self {
        let c = config::load_calibration();

        // If possible load calibration, if not, set it to default.
        match c {
            Ok(c) => c,
            Err(_) => config::default(),
        }
    }

    fn load_calibration() -> Result<Self, serde_yaml::Error> {
        let config_data = std::fs::read_to_string(CONFIG_PATH).expect("Unable to read file");
        serde_yaml::from_str(&config_data)
    }

    pub fn save_calibration(&self) {
        let yaml = serde_yaml::to_string(&self).expect("Unable to convert config to YAML.");

        #[cfg(debug_assertions)]
        {
            println!("File Path:{:?}", CONFIG_PATH);
            println!("Config string:{:?}", yaml);
        }

        std::fs::write(CONFIG_PATH, yaml).expect("Unable to write to file.");
    }

    fn calibrate(&mut self) {}
}

impl Default for config {
    fn default() -> Self {
        config {
            _calibration: calibration {
                x_offset: 250i32,
                y_offset: 400i32,

                x_offset_multiplicator: 0.25f32,
                y_offset_multiplicator: 0.35f32,

                x_speed_multiplicator: 10f32 / 5f32,
                y_speed_multiplicator: 10f32 / 6.5f32,
            },
            _mode: mode::default(),
        }
    }
}
