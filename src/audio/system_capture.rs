use crate::audio::AudioSource;
use ruhear::RUHear;
use std::sync::{Arc, Mutex};

pub struct SystemCapture {
    name: String,
    ruhear: RUHear,
}

impl SystemCapture {
    pub fn new(callback: Box<dyn FnMut(Vec<Vec<f32>>) + Send>) -> Self {
        let name = "System Default Output".to_string();
        let callback = Arc::new(Mutex::new(callback));
        let ruhear = RUHear::new(callback.clone());
        Self { name, ruhear }
    }
}

impl AudioSource for SystemCapture {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn start(&mut self) {
        let _ = self.ruhear.start();
    }
    fn stop(&mut self) {
        let _ = self.ruhear.stop();
    }
}
