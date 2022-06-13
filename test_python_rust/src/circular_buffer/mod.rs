// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use math::round;

// Size of the circular buffer.
const DATA_MAX_SIZE: usize = 4;

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
        if self._index >= self._data.capacity() {
            self._index = 0usize;
        }

        if self._data.len() <= self._index {
            self._data.push(data);
        } else {
            self._data[self._index] = data;
        }

        self._index += 1;
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
}

impl Default for circular_buffer {
    fn default() -> Self {
        circular_buffer {
            _data: Vec::with_capacity(DATA_MAX_SIZE),
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
