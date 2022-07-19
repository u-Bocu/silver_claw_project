// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use math::round;

// Size of the circular buffer.
const BUFFER_DEFAULT_SIZE: usize = 4usize;

const BUFFER_MAX_SIZE: usize = 20usize;
const BUFFER_MIN_SIZE: usize = 3usize;

const ACCELERATION_HI_HARDCAP: i32 = 100i32;
const ACCELERATION_HI_SOFTCAP: i32 = 20i32;
const ACCELERATION_LO_SOFTCAP: i32 = 10i32;

pub struct circular_buffer {
    _data: Vec<(i32, i32)>,
    _index: usize,
}

impl circular_buffer {
    pub fn new(s: usize) -> Self {
        circular_buffer {
            _data: Vec::with_capacity(s),
            _index: 0usize,
        }
    }

    pub fn insert(&mut self, data: (i32, i32)) {
        self._data.insert(0, data)
    }

    /**
     * Condidering cursor acceleration, this functions resizes the buffer to get either precision of responsivity.
     *
     * No parameters.
     * No return variable.
     */
    pub fn reevaluate_size(&mut self) {
        let mut acceleration;

        match self.get_accelerations() {
            Some(a) => {
                acceleration = a;

                let last_acceleration: (i32, i32) = acceleration.pop().unwrap();
                let last_acceleration_flat: i32 =
                    (last_acceleration.0).abs() + (last_acceleration.1).abs();

                if last_acceleration_flat > ACCELERATION_HI_HARDCAP {
                    self.resize(BUFFER_MIN_SIZE)
                } else if last_acceleration_flat > ACCELERATION_HI_SOFTCAP {
                    let mut new_acceleration: usize = self._data.capacity() - 3;
                    if new_acceleration < BUFFER_MIN_SIZE {
                        new_acceleration = BUFFER_MIN_SIZE;
                    }

                    self.resize(new_acceleration)
                } else if last_acceleration_flat < ACCELERATION_LO_SOFTCAP {
                    let mut new_acceleration: usize = self._data.capacity() + 1;
                    if new_acceleration > BUFFER_MAX_SIZE {
                        new_acceleration = BUFFER_MAX_SIZE;
                    }

                    self.resize(new_acceleration)
                }
            }
            None => {}
        };
    }

    /**
     * Modifies circular_buffer's data capacity to increase reactivity or filter accuracy.
     * Lower buffer size means more reactivity, higher buffer size means more accuracy for small movements.
     *
     * Parameter: the desired new buffer size
     */
    pub fn resize(&mut self, mut s: usize) {
        if s > BUFFER_MAX_SIZE {
            s = BUFFER_MAX_SIZE;
        } else if s < BUFFER_MIN_SIZE {
            s = BUFFER_MIN_SIZE;
        }

        if s > self._data.capacity() {
            self._data.reserve_exact(s - self._data.len());
        } else if s < self._data.capacity() {
            self._data = Vec::with_capacity(s);
        }
    }

    /**
     * No parameter.
     * Returns a vector containing last cursor speed (up to buffer size - 1).
     */
    fn get_speeds(&self) -> Option<Vec<(i32, i32)>> {
        let mut speeds: Vec<(i32, i32)> = self._data.clone();

        if speeds.len() > 1 {
            for i in 0..(speeds.len() - 1usize) {
                let tmp: (i32, i32) = (
                    (speeds[i].0 - speeds[i + 1].0).abs(),
                    (speeds[i].1 - speeds[i + 1].1).abs(),
                );

                speeds[i] = tmp;
            }
            speeds.pop();

            Some(speeds)
        } else {
            None
        }
    }

    /**
     * No parameter.
     * Returns a vector containing last cursor accelerations (up to buffer size - 2).
     */
    fn get_accelerations(&self) -> Option<Vec<(i32, i32)>> {
        let accelerations: Option<Vec<(i32, i32)>> = self.get_speeds();

        match accelerations {
            Some(mut accelerations) => {
                if accelerations.len() > 1 {
                    for i in 0..(accelerations.len() - 1usize) {
                        let tmp: (i32, i32) = (
                            (accelerations[i].0 - accelerations[i + 1].0).abs(),
                            (accelerations[i].1 - accelerations[i + 1].1).abs(),
                        );

                        accelerations[i] = tmp;
                    }
                    accelerations.pop();

                    Some(accelerations)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /**
     * Returns the acceleration considering the median speed stored in the buffer and the last speed calculated.
     */
    fn get_acceleration(&self) -> Option<i32> {
        let speeds: Option<Vec<(i32, i32)>> = self.get_speeds();

        match speeds {
            Some(speeds) => {
                let speeds_size: f64 = speeds.len() as f64;

                if speeds_size > 1f64 {
                    let acceleration: i32 =
                        (speeds[round::floor(speeds_size / 2f64, 0i8) as usize].0
                            - speeds.last().unwrap().0)
                            .abs()
                            + (speeds[round::floor(speeds_size / 2f64, 0i8) as usize].1
                                - speeds.last().unwrap().1)
                                .abs();
                    Some(acceleration)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /**
     * Applies a mean filter to the circular buffer values.
     * Returns the tuple of coordinates calculated with the mean filter.
     */
    pub fn mean_filter(&mut self) -> (i32, i32) {
        let mut sum: (i32, i32) = (0i32, 0i32);

        for data in &self._data {
            sum.0 += data.0;
            sum.1 += data.1;
        }

        let size: i32 = self._data.len() as i32;
        let res: (i32, i32) = (sum.0 / size, sum.1 / size);

        res
    }

    /**
     * Applies a median filter to the circular buffer values.
     * Returns the tuple of coordinates calculated with the median filter.
     */
    pub fn median_filter(&mut self) -> (i32, i32) {
        let mut x: Vec<i32> = Vec::with_capacity(self._data.len());
        let mut y: Vec<i32> = Vec::with_capacity(self._data.len());

        for data in &self._data {
            x.push(data.0);
            y.push(data.1);
        }

        (array_median(&mut x), array_median(&mut y))
    }

    /**
     * Print current acceleration.
     * Use for debug purpose.
     */
    pub fn print_acceleration(&self) {
        let acceleration: Option<i32> = self.get_acceleration();

        match acceleration {
            Some(a) => {
                println!("{:?}", a)
            }
            None => {}
        }
    }
}

impl Default for circular_buffer {
    fn default() -> Self {
        circular_buffer {
            _data: Vec::with_capacity(BUFFER_DEFAULT_SIZE),
            _index: 0usize,
        }
    }
}

/**
 * Finds and returns the median value in an array.
 */
fn array_median(v: &mut Vec<i32>) -> i32 {
    v.sort();

    if v.len() % 2 == 0 {
        (v[((v.len() / 2) - 1) as usize] + v[(v.len() / 2) as usize]) / 2
    } else {
        v[round::floor((v.len() / 2) as f64, 0i8) as usize]
    }
}
