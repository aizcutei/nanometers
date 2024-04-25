use rustfft::{num_complex::Complex64, FftPlanner};
use std::f64::consts::PI;

const FS: f64 = 48000.0;

fn hann(len: usize) -> Vec<f32> {
    (0..len)
        .map(|i| {
            let x = (i as f64) / (len as f64 - 1.0);
            0.5 * (1.0 - (2.0 * PI * x).cos()) as f32
        })
        .collect()
}

fn timederivhann(len: isize) -> Vec<f64> {
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

fn timeramphann(len: isize) -> Vec<f64> {
    let m = len / 2;
    let tramp = (-m..m).map(|x| (x as f64) + 0.5).collect::<Vec<f64>>();
    (0..len)
        .map(|i| {
            let x = (i as f64) / (len as f64 - 1.0);
            0.5 * (1.0 - (2.0 * PI * x).cos()) * tramp[i as usize] / FS
        })
        .collect()
}

// pub const HANN_1024: [f32; 1024] = hann::<1024>();
// pub const HANN_2048: [f32; 2048] = hann::<2048>();
// pub const HANN_DT_1024: [f32; 1024] = timederivhann(1024);
// pub const HANN_T_1024: [f32; 1024] = timeramphann(1024);

// pub const HANN_2048: [f32; 2048] = hann(2048);
// pub const HANN_DT_2048: [f32; 2048] = timederivhann(2048);
// pub const HANN_T_2048: [f32; 2048] = timeramphann(2048);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hann() {
        let hann = hann(10);
        let hann_10 = [
            0.0,
            0.116977778440511,
            0.4131759111665348,
            0.7499999999999999,
            0.9698463103929542,
            0.9698463103929542,
            0.7500000000000002,
            0.413175911166535,
            0.1169777784405111,
            0.0,
        ];
        assert_eq!(hann.len(), 10);
        assert_eq!(hann, hann_10);
    }

    #[test]
    fn test_timederivhann() {
        let timederivhann = timederivhann(10);
        let timederivhann_10 = vec![
            301.49629623273296,
            1627.4648199448773,
            2669.5592277383735,
            2280.0969596899877,
            936.5062556637607,
            -936.5062556637583,
            -2280.096959689988,
            -2669.5592277383735,
            -1627.464819944879,
            -301.49629623273324,
        ];
        assert_eq!(timederivhann.len(), 10);
        assert_eq!(timederivhann, timederivhann_10);
    }

    #[test]
    fn test_timeramphann() {
        let timeramphann = timeramphann(10);
        let timeramphann_10 = vec![
            0.0,
            -8.529629677953926e-6,
            -2.1519578706590354e-5,
            -2.3437499999999994e-5,
            -1.0102565733259939e-5,
            1.0102565733259939e-5,
            2.343750000000001e-5,
            2.1519578706590368e-5,
            8.529629677953936e-6,
            0.0,
        ];
        assert_eq!(timeramphann.len(), 10);
        assert_eq!(timeramphann, timeramphann_10);
    }
}
