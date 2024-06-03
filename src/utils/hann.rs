use rustfft::{num_complex::Complex64, FftPlanner};
use std::f64::consts::PI;

const FS: f64 = 48000.0;

pub fn hann(len: usize) -> Vec<f32> {
    (0..len)
        .map(|i| {
            let x = (i as f64) / (len as f64 - 1.0);
            0.5 * (1.0 - (2.0 * PI * x).cos()) as f32
        })
        .collect()
}

pub fn timederivhann(len: isize) -> Vec<f64> {
    let m = len / 2;
    let pos: Vec<f64> = (0..m).map(|x| (x as f64) + 0.5).collect();
    let neg: Vec<f64> = ((-m)..0).map(|x| (x as f64) + 0.5).collect();
    let mut framp: Vec<f64> = Vec::with_capacity(len as usize);
    framp.extend(pos.iter().map(|&x| x / (len as f64)));
    framp.extend(neg.iter().map(|&x| x / (len as f64)));
    let fft = FftPlanner::<f64>::new().plan_fft_forward(len as usize);
    let mut sample: Vec<Complex64> = (0..len as usize)
        .map(|i| {
            Complex64::new(
                0.5 * (1.0 - (2.0 * PI * (i as f64) / (len as f64 - 1.0)).cos()),
                0.0,
            )
        })
        .collect();
    fft.process(&mut sample);
    let mut result: Vec<Complex64> = (0..len as usize)
        .map(|i| Complex64::new(sample[i].re * framp[i], sample[i].im * framp[i]))
        .collect();
    let ifft = FftPlanner::<f64>::new().plan_fft_inverse(len as usize);
    ifft.process(&mut result);
    result.iter().map(|x| -x.im * FS / len as f64).collect()
}

pub fn timeramphann(len: isize) -> Vec<f64> {
    let m = len / 2;
    let tramp = (-m..m).map(|x| (x as f64) + 0.5).collect::<Vec<f64>>();
    (0..len)
        .map(|i| {
            let x = (i as f64) / (len as f64 - 1.0);
            0.5 * (1.0 - (2.0 * PI * x).cos()) * tramp[i as usize] / FS
        })
        .collect()
}

pub fn framesequence(len: usize) -> Vec<f64> {
    let mut result = vec![];
    for i in 0..len {
        result.push(i as f64 / len as f64 * FS);
    }
    result
}
