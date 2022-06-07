// An attribute to hide warnings for unused code.
#![allow(dead_code)]
// An attribute to allow non CamelCase and let snake_case be default convention.
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

const DATA_MAX_SIZE: usize = 3;

pub struct circular_buffer {
    _data: Vec<(f32, f32)>,
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
    fn append(&mut self, data: (f32, f32)) {
        if self._index >= self._data.capacity() {
            self._index = 0usize;
        }

        self._data[self._index] = data;
        self._index += 1;
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
