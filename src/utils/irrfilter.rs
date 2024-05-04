pub fn shelving_filter(x: &Vec<f32>, y: &mut Vec<f32>) {
    let a1 = -1.69065929318241;
    let a2 = 0.73248077421585;
    let b0 = 0.73248077421585;
    let b1 = -2.69169618940638;
    let b2 = 1.19839281085285;
    (0..x.len()).for_each(|i| {
        if i < 2 {
            y[i] = 0.0;
        } else {
            y[i] = b0 * x[i] + b1 * x[i - 1] + b2 * x[i - 2] + a1 * y[i - 1] + a2 * y[i - 2];
        }
    });
}

pub fn highpass_filter(x: &Vec<f32>, y: &mut Vec<f32>) {
    let a1 = -1.99004745483398;
    let a2 = 0.99007225036621;
    let b0 = 1.0;
    let b1 = -2.0;
    let b2 = 1.0;
    (0..x.len()).for_each(|i| {
        if i < 2 {
            y[i] = 0.0;
        } else {
            y[i] = b0 * x[i] + b1 * x[i - 1] + b2 * x[i - 2] + a1 * y[i - 1] + a2 * y[i - 2];
        }
    });
}
