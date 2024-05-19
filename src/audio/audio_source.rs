pub trait AudioSource {
    fn get_name(&self) -> String;
    fn start(&mut self);
    fn stop(&mut self);
}
