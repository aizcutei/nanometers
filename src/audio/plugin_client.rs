use crate::AudioSource;
use interprocess::local_socket::{LocalSocketStream, NameTypeSupport};
use std::io::Read;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
pub const RING_BUFFER_SIZE: usize = 48000;

#[cfg(not(target_os = "windows"))]
pub const RING_BUFFER_SIZE: usize = 15360;

pub struct PluginClient {
    name: String,
    handle: Option<thread::JoinHandle<()>>,
    callback: Arc<Mutex<dyn FnMut(Vec<Vec<f32>>) + Send>>,
    stop_sender: mpsc::Sender<()>,
    stop_receiver: Arc<Mutex<mpsc::Receiver<()>>>,
}

impl PluginClient {
    pub fn new(callback: Box<dyn FnMut(Vec<Vec<f32>>) + Send>) -> Self {
        let name = "Plugin Server".to_string();
        let (tx, rx) = mpsc::channel();
        Self {
            name,
            handle: None,
            callback: Arc::new(Mutex::new(callback)),
            stop_sender: tx,
            stop_receiver: Arc::new(Mutex::new(rx)),
        }
    }
}

impl AudioSource for PluginClient {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn start(&mut self) {
        let name = {
            use NameTypeSupport::*;
            match NameTypeSupport::query() {
                OnlyPaths => "/tmp/nanometers.sock",
                OnlyNamespaced | Both => "@nanometers.sock",
            }
        };
        let callback = self.callback.clone();
        let stop_receiver = self.stop_receiver.clone();

        let handle = thread::spawn(move || {
            let stop_receiver = stop_receiver.lock().unwrap();
            let mut buf = vec![0u8; (RING_BUFFER_SIZE + 1) * 4 as usize];
            let mut ring_buf_last_idx: usize = 0;
            loop {
                match stop_receiver.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    _ => {}
                }

                let mut conn = match LocalSocketStream::connect(name) {
                    Ok(conn) => conn,
                    Err(_e) => {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                };
                let mut update_buf: Vec<f32> = Vec::new();
                match conn.read(&mut buf) {
                    Ok(_) => {
                        let buffer = buf
                            .chunks_exact(4)
                            .map(|chunk| {
                                f32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                            })
                            .collect::<Vec<f32>>();
                        let ring_buf_this_idx = buffer[0] as usize;
                        if ring_buf_this_idx > ring_buf_last_idx {
                            update_buf = buffer[ring_buf_last_idx..ring_buf_this_idx].to_vec();
                        } else {
                            update_buf = buffer[ring_buf_last_idx..].to_vec();
                            update_buf.extend_from_slice(&buffer[..ring_buf_this_idx]);
                        }
                        ring_buf_last_idx = ring_buf_this_idx;

                        let right_buf = update_buf.split_off(update_buf.len() / 2);
                        let update_buf = vec![update_buf, right_buf];

                        let mut callback = callback.lock().unwrap();
                        callback(update_buf);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server: {}", e);
                        continue;
                    }
                }
            }
        });
        self.handle = Some(handle);
    }

    fn stop(&mut self) {
        self.stop_sender.send(()).unwrap();
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}
