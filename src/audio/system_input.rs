#![allow(unused)]
use crate::audio::AudioSource;
// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub struct SystemInput {
    name: String,
    callback: Arc<Mutex<dyn FnMut(Vec<Vec<f32>>) + Send>>,
    host: cpal::Host,
    device: cpal::Device,
    format: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl SystemInput {
    pub fn new(callback: Box<dyn FnMut(Vec<Vec<f32>>) + Send>) -> Self {
        let name = "System Default Microphone".to_string();
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();
        let format = device.default_input_config().unwrap();
        Self {
            name,
            callback: Arc::new(Mutex::new(callback)),
            host,
            device,
            format,
            stream: None,
        }
    }
}

impl AudioSource for SystemInput {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self) {
        if self.stream.is_none() {
            let callback = self.callback.clone();
            let channels = &self.format.channels().clone();
            let channels = *channels as usize;
            let stream = match self.format.sample_format() {
                cpal::SampleFormat::F32 => self.device.build_input_stream(
                    &self.format.config(),
                    move |data: &[f32], &_| {
                        let mut bufs = vec![vec![]; channels];
                        for (i, sample) in data.chunks(channels).enumerate() {
                            for (j, &channel) in sample.iter().enumerate() {
                                bufs[j].push(channel as f32);
                            }
                        }
                        if let Ok(mut callback) = callback.lock() {
                            (*callback)(bufs);
                        }
                    },
                    |err| {
                        eprintln!("an error occurred on stream: {}", err);
                    },
                    None,
                ),
                sample_format => {
                    panic!("unsupported format {:?}", sample_format);
                }
            }
            .unwrap();
            self.stream = Some(stream);
        }
        if let Some(stream) = &self.stream {
            stream.play();
        }
    }

    fn stop(&mut self) {
        if let Some(stream) = &self.stream {
            stream.pause().unwrap();
        }
    }
}
