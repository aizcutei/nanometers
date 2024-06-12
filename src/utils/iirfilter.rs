use crate::setting::*;

// LUFS filter, ITU-R BS.1770-5, high-pass 100Hz, shelving 1000Hz
pub const HP100_A: [f32; 2] = [-1.9900474548339844, 0.9900722503662109];
pub const HP100_B: [f32; 3] = [1.0, -2.0, 1.0];

pub const SHELVING_A: [f32; 2] = [-1.69065929318241, 0.73248077421585];
pub const SHELVING_B: [f32; 3] = [1.53512485958697, -2.69169618940638, 1.19839281085285];

// Multiband filter, 200Hz, 2000Hz
pub const LP200_A: [f32; 2] = [-1.9629800893893397, 0.9636529842237055];
pub const LP200_B: [f32; 3] = [
    0.00016822370859146955,
    0.0003364474171829391,
    0.00016822370859146955,
];

pub const HP200_A: [f32; 2] = [-1.9629800893893397, 0.9636529842237055];
pub const HP200_B: [f32; 3] = [0.9816582684032612, -1.9633165368065224, 0.9816582684032612];

pub const LP2000_A: [f32; 2] = [-1.632993161855452, 0.6905989232414969];
pub const LP2000_B: [f32; 3] = [
    0.014401440346511215,
    0.02880288069302243,
    0.014401440346511215,
];

pub const HP2000_A: [f32; 2] = [-1.632993161855452, 0.6905989232414969];
pub const HP2000_B: [f32; 3] = [0.8308980212742374, -1.6617960425484748, 0.8308980212742374];

pub fn lufs_combined_filter(x_0: f32, buffer: &mut IIRBuffer) -> f32 {
    let y_0 = SHELVING_B[0] * x_0 + SHELVING_B[1] * buffer.x_1 + SHELVING_B[2] * buffer.x_2
        - SHELVING_A[0] * buffer.y_1
        - SHELVING_A[1] * buffer.y_2;
    let z_0 = y_0 - buffer.y_1 - buffer.y_1 + buffer.y_2
        - HP100_A[0] * buffer.z_1
        - HP100_A[1] * buffer.z_2;
    buffer.x_2 = buffer.x_1;
    buffer.x_1 = x_0;
    buffer.y_2 = buffer.y_1;
    buffer.y_1 = y_0;
    buffer.z_2 = buffer.z_1;
    buffer.z_1 = z_0;
    z_0
}

pub fn multiband_low_filter(x_0: f32, buffer: &mut IIRBuffer) -> f32 {
    let y_0 = LP200_B[0] * x_0 + LP200_B[1] * buffer.x_1 + LP200_B[2] * buffer.x_2
        - LP200_A[0] * buffer.y_1
        - LP200_A[1] * buffer.y_2;
    buffer.x_2 = buffer.x_1;
    buffer.x_1 = x_0;
    buffer.y_2 = buffer.y_1;
    buffer.y_1 = y_0;
    y_0
}

pub fn multiband_mid_filter(x_0: f32, buffer: &mut IIRBuffer) -> f32 {
    let y_0 = HP200_B[0] * x_0 + HP200_B[1] * buffer.x_1 + HP200_B[2] * buffer.x_2
        - HP200_A[0] * buffer.y_1
        - HP200_A[1] * buffer.y_2;
    let z_0 = LP2000_B[0] * y_0 + LP2000_B[1] * buffer.y_1 + LP2000_B[2] * buffer.y_2
        - LP2000_A[0] * buffer.z_1
        - LP2000_A[1] * buffer.z_2;
    buffer.x_2 = buffer.x_1;
    buffer.x_1 = x_0;
    buffer.y_2 = buffer.y_1;
    buffer.y_1 = y_0;
    buffer.z_2 = buffer.z_1;
    buffer.z_1 = z_0;
    z_0
}

pub fn multiband_high_filter(x_0: f32, buffer: &mut IIRBuffer) -> f32 {
    let y_0 = HP2000_B[0] * x_0 + HP2000_B[1] * buffer.x_1 + HP2000_B[2] * buffer.x_2
        - HP2000_A[0] * buffer.y_1
        - HP2000_A[1] * buffer.y_2;
    buffer.x_2 = buffer.x_1;
    buffer.x_1 = x_0;
    buffer.y_2 = buffer.y_1;
    buffer.y_1 = y_0;
    y_0
}
