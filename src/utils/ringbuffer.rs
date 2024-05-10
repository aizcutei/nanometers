use std::{collections::VecDeque, marker::PhantomData};

#[derive(Debug)]
pub struct RingBuffer {
    buffer: VecDeque<f32>,
    capacity: usize,
    _marker: PhantomData<*const ()>,
}

impl RingBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(size),
            capacity: size,
            _marker: PhantomData,
        }
    }

    pub fn new_with_default(size: usize, default: f32) -> Self {
        Self {
            buffer: VecDeque::from(vec![default; size]),
            capacity: size,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: f32) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    pub fn push_slice(&mut self, slice: &[f32]) {
        if slice.len() + self.buffer.len() > self.capacity {
            let diff = slice.len() + self.buffer.len() - self.capacity;
            for i in 0..diff {
                self.buffer.pop_front();
                self.buffer.push_back(slice[i]);
            }
        }
    }

    pub fn get(&self, index: usize) -> f32 {
        self.buffer.get(index).unwrap_or(&0.0).clone()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

unsafe impl Send for RingBuffer {}
unsafe impl Sync for RingBuffer {}
