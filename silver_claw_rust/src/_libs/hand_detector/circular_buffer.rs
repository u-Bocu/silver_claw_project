#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use math::round;

// Size of the circular buffer.
const BUFFER_DEFAULT_SIZE: usize = 4usize;

const BUFFER_MAX_SIZE: usize = 20usize;
const BUFFER_MIN_SIZE: usize = 3usize;

const ACCELERATION_HI_HARDCAP: i32 = 300i32;
const ACCELERATION_HI_SOFTCAP: i32 = 30i32;
const ACCELERATION_LO_SOFTCAP: i32 = 10i32;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn append(&mut self, data: (i32, i32)) {
        if self._data.len() <= self._index {
            self._data.push(data);
        } else {
            self._data[self._index] = data;
        }

        self._index += 1;

        if self._index >= self._data.capacity() {
            self._index = 0usize;
        }
    }

    pub fn clear(&mut self) {
        self._data.clear();
        self.resize(BUFFER_DEFAULT_SIZE);
    }

    // TODO: Find a better function to reevalute size.

    /**
     * Condidering cursor acceleration, this functions resizes the buffer to get either precision or responsivity.
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

                if last_acceleration_flat > ACCELERATION_HI_SOFTCAP {
                    let buffer_shrinking: usize = last_acceleration_flat as usize / 10usize;
                    let mut new_buffer_size: usize = match buffer_shrinking > self._data.capacity()
                    {
                        true => BUFFER_MIN_SIZE,
                        false => self._data.capacity() - buffer_shrinking,
                    };

                    if new_buffer_size < BUFFER_MIN_SIZE {
                        new_buffer_size = BUFFER_MIN_SIZE;
                    }

                    self.resize(new_buffer_size)
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
     * Parameter: the desired new buffer size.
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
            if s <= self._data.len() {
                /*
                 * We need to purge obsolete values in a smart way.
                 * We use Vec::drain for that.
                 * The hard part is calculating range indexes for drain.
                 */
                let index: usize = match self._index >= self._data.len() {
                    true => self._data.len() - 1,
                    false => match self._index {
                        0 => self._data.len() - 1,
                        _ => self._index - 1,
                    },
                };

                let mut i: i32 = index as i32;

                for _j in 0..s {
                    i -= 1;
                    if i < 0 {
                        i = (self._data.len() as i32) - 1;
                    }
                }

                let i: usize = i as usize;

                if i < index {
                    self._data = self._data.drain(i..=index).collect();
                } else if i > index {
                    self._data.drain(index + 1..i);
                } else {
                    // i == index
                    let tmp: (i32, i32) = self._data.remove(i);
                    self._data.clear();
                    self._data.push(tmp);
                }
            }

            self._data.shrink_to(s);
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

    pub fn get_shift(&self) -> Option<(i32, i32)> {
        let speeds: Option<Vec<(i32, i32)>> = self.get_speeds();

        match speeds {
            Some(speeds) => {
                let speeds_size: f64 = speeds.len() as f64;

                if speeds_size > 1f64 {
                    let acceleration = (
                        (speeds[round::floor(speeds_size / 2f64, 0i8) as usize].0
                            - speeds.last().unwrap().0),
                        (speeds[round::floor(speeds_size / 2f64, 0i8) as usize].1
                            - speeds.last().unwrap().1),
                    );
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
        if size == 0 {
            (0i32, 0i32)
        } else {
            (sum.0 / size, sum.1 / size)
        }
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

    if v.len() == 0 {
        0i32
    } else if v.len() % 2 == 0 {
        (v[((v.len() / 2) - 1) as usize] + v[(v.len() / 2) as usize]) / 2
    } else {
        v[round::floor((v.len() / 2) as f64, 0i8) as usize]
    }
}
