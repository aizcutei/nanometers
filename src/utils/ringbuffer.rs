use std::marker::PhantomData;

#[derive(Debug)]
pub struct RingBuffer {
    buffer: Vec<f32>,
    index: usize,
    _marker: PhantomData<*const ()>,
}

impl RingBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            index: 0,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: Vec<f32>) -> usize {
        let len = value.len();
        let size = self.buffer.len();
        if len > size {
            return 0;
        }
        if self.index + len <= size {
            self.buffer[self.index..self.index + len].copy_from_slice(&value);
            self.index += len;
        } else {
            let split_index = size - self.index;
            let remain_len = len - split_index;
            self.buffer[self.index..size].copy_from_slice(&value[..split_index]);
            self.buffer[..remain_len].copy_from_slice(&value[split_index..]);
            self.index = remain_len;
        }
        self.index
    }

    pub fn get(&self, len: usize) -> Vec<f32> {
        let size = self.buffer.len();
        if len > size {
            return vec![0.0; 0];
        }
        let mut value = vec![];
        if self.index >= len {
            value.extend_from_slice(&self.buffer[self.index - len..self.index]);
        } else {
            let split_index = size - len;
            value.extend_from_slice(&self.buffer[..self.index]);
            value.extend_from_slice(&self.buffer[split_index..size]);
        }
        value
    }

    pub fn index(&self) -> usize {
        self.index.clone()
    }
}

unsafe impl Send for RingBuffer {}
unsafe impl Sync for RingBuffer {}
