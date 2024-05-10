#[derive(Debug, Clone, Default)]
pub struct RawData {
    pub l: Vec<f32>,
    pub r: Vec<f32>,
    pub m: Vec<f32>,
    pub s: Vec<f32>,
}

impl RawData {
    pub fn new() -> Self {
        Self {
            l: vec![],
            r: vec![],
            m: vec![],
            s: vec![],
        }
    }

    pub fn concat(&mut self, data: RawData) {
        self.l.extend(data.l);
        self.r.extend(data.r);
        self.m.extend(data.m);
        self.s.extend(data.s);
    }

    pub fn concat_front(&mut self, data: RawData) {
        self.l.splice(0..0, data.l);
        self.r.splice(0..0, data.r);
        self.m.splice(0..0, data.m);
        self.s.splice(0..0, data.s);
    }

    pub fn split_index(&self, index: usize) -> RawData {
        let l = self.l[index..].to_vec();
        let r = self.r[index..].to_vec();
        let m = self.m[index..].to_vec();
        let s = self.s[index..].to_vec();
        RawData { l, r, m, s }
    }

    pub fn clear(&mut self) {
        self.l.clear();
        self.r.clear();
        self.m.clear();
        self.s.clear();
    }

    pub fn push_l(&mut self, value: f32) {
        self.l.push(value);
    }

    pub fn push_r(&mut self, value: f32) {
        self.r.push(value);
    }

    pub fn push_m(&mut self, value: f32) {
        self.m.push(value);
    }

    pub fn push_s(&mut self, value: f32) {
        self.s.push(value);
    }

    pub fn extend_l(&mut self, value: &[f32]) {
        self.l.extend_from_slice(value);
    }

    pub fn extend_r(&mut self, value: &[f32]) {
        self.r.extend_from_slice(value);
    }

    pub fn extend_m(&mut self, value: &[f32]) {
        self.m.extend_from_slice(value);
    }

    pub fn extend_s(&mut self, value: &[f32]) {
        self.s.extend_from_slice(value);
    }
}
