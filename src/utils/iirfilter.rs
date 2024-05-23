use crate::setting::*;

pub const SHELVING_A1: f32 = -1.69065929318241;
pub const SHELVING_A2: f32 = 0.73248077421585;
pub const SHELVING_B0: f32 = 1.53512485958697;
pub const SHELVING_B1: f32 = -2.69169618940638;
pub const SHELVING_B2: f32 = 1.19839281085285;
pub const HIGHPASS_A1: f32 = -1.99004745483398;
pub const HIGHPASS_A2: f32 = 0.99007225036621;
pub const HIGHPASS_B0: f32 = 1.0;
pub const HIGHPASS_B1: f32 = -2.0;
pub const HIGHPASS_B2: f32 = 1.0;

pub fn combined_filter(x_0: f32, buffer: &mut IIRBuffer) -> f32 {
    let y_0 = SHELVING_B0 * x_0 + SHELVING_B1 * buffer.x_1 + SHELVING_B2 * buffer.x_2
        - SHELVING_A1 * buffer.y_1
        - SHELVING_A2 * buffer.y_2;
    let z_0 = HIGHPASS_B0 * y_0 + HIGHPASS_B1 * buffer.y_1 + HIGHPASS_B2 * buffer.y_2
        - HIGHPASS_A1 * buffer.z_1
        - HIGHPASS_A2 * buffer.z_2;
    buffer.x_2 = buffer.x_1;
    buffer.x_1 = x_0;
    buffer.y_2 = buffer.y_1;
    buffer.y_1 = y_0;
    buffer.z_2 = buffer.z_1;
    buffer.z_1 = z_0;
    z_0
}
