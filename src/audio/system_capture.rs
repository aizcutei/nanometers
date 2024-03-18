use crate::AudioSource;
use ruhear::RUHear;
use std::sync::{Arc, Mutex};

pub struct SystemCapture {
    name: String,
    ruhear: RUHear,
    callback: Arc<Mutex<dyn FnMut(Vec<Vec<f32>>) + Send>>,
}

impl SystemCapture {
    pub fn new(callback: fn(Vec<Vec<f32>>)) -> Self {
        let name = "System Default Output".to_string();
        let ruhear = RUHear::new(Arc::new(Mutex::new(callback)));
        Self {
            name,
            ruhear,
            callback: Arc::new(Mutex::new(callback)),
        }
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
