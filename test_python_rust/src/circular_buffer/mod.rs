// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use pyo3::prelude::*;

const DATA_MAX_SIZE: usize = 3;

pub struct circular_buffer {
    _data: Vec<(i32, i32)>,
    _index: usize,
}

impl circular_buffer {
    fn new(s: usize) -> Self {
        circular_buffer {
            _data: Vec::with_capacity(s),
            _index: 0usize,
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn median_filter(&mut self) -> PyResult<(i32, i32)> {
        let mut sum: (i32, i32) = (0i32, 0i32);

        for data in &self._data {
            sum.0 += data.0;
            sum.1 += data.1;
        }

        let size: i32 = self._data.len() as i32;
        let res: (i32, i32) = (sum.0 / size, sum.1 / size);

        Ok(res)
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
