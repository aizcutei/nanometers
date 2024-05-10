#![allow(dead_code)]
// use crate::AudioSource;
// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct SystemInput {
    name: String,
    callback: Arc<Mutex<dyn FnMut(Vec<Vec<f32>>) + Send>>,
    stream: cpal::Stream,
}
