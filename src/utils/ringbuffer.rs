use std::{collections::VecDeque, marker::PhantomData};

#[derive(Debug, Default)]
pub struct RingBufferF32 {
    buffer: VecDeque<f32>,
    capacity: usize,
    _marker: PhantomData<*const ()>,
}

impl RingBufferF32 {
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

unsafe impl Send for RingBufferF32 {}
unsafe impl Sync for RingBufferF32 {}

#[derive(Debug, Default)]
pub struct RingBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    _marker: PhantomData<*const ()>,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(size),
            capacity: size,
            _marker: PhantomData,
        }
    }

    pub fn new_with_default(size: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            buffer: VecDeque::from(vec![default; size]),
            capacity: size,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    pub fn push_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        if slice.len() + self.buffer.len() > self.capacity {
            let diff = slice.len() + self.buffer.len() - self.capacity;
            for i in 0..diff {
                self.buffer.pop_front();
                self.buffer.push_back(slice[i].clone());
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T>
    where
        T: Clone,
    {
        self.buffer.get(index)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}
