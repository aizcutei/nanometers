use crate::utils::*;

use crossbeam_channel::Sender;
use egui::Pos2;
use realfft::RealFftPlanner;
use rustfft::num_traits::{real::Real, Pow};
use std::sync::{Arc, Mutex};

const SQRT_2: f32 = 1.4142135;

pub fn get_callback(
    tx: Sender<SendData>,
    buffer: Arc<Mutex<AudioSourceBuffer>>,
) -> Box<dyn FnMut(Vec<Vec<f32>>) + Send + Sync> {
    Box::new(move |data: Vec<Vec<f32>>| {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("callback");

        let mut buffer = buffer.lock().unwrap();
        let waveform_block_length = 256;
        let stereo_block_length = 1;

        let mut send_data = SendData::new();
        let len = data[0].len();
        let mut amp_l = 0.0;
        let mut amp_r = 0.0;
        for i in 0..len {
            let l = data[0][i];
            let r = data[1][i];
            let m = (l + r) / 2.0;
            let s = (l - r) / 2.0;
            // Stereo
            buffer.stereo.update(l, r);
            if buffer.stereo.index >= stereo_block_length {
                buffer.stereo.index = 0;
                send_data.stereo_data.lissa.push(Pos2::new(l, r));
                send_data
                    .stereo_data
                    .linear
                    .push(Pos2::new(-SQRT_2 * s, -SQRT_2 * m));

                let length = (l * l + r * r).sqrt();
                let log_x = if length.log10() >= -3.0 {
                    (length.log10() / 3.0 + 1.0) * l / length
                } else {
                    0.0
                };
                let log_y = if length.log10() >= -3.0 {
                    (length.log10() / 3.0 + 1.0) * r / length
                } else {
                    0.0
                };
                send_data.stereo_data.log.push(Pos2::new(
                    0.7071067812 * (log_y - log_x),
                    -0.7071067812 * (log_x + log_y),
                ));
            }

            // Waveform
            buffer.waveform.update_l(l);
            buffer.waveform.update_r(r);
            buffer.waveform.update_m(m);
            buffer.waveform.update_s(s);
            buffer.waveform.index += 1;
            if buffer.waveform.index >= waveform_block_length {
                let mut waveform_buffer = buffer.waveform.clone();
                buffer.waveform.reset();
                send_data.waveform_data.l.push(waveform_buffer.l);
                send_data.waveform_data.r.push(waveform_buffer.r);
                send_data.waveform_data.m.push(waveform_buffer.m);
                send_data.waveform_data.s.push(waveform_buffer.s);

                let mut real_planner = RealFftPlanner::<f32>::new();
                let r2c = real_planner.plan_fft_forward(waveform_block_length);
                let mut spectrum = r2c.make_output_vec();
                r2c.process(&mut waveform_buffer.raw.l, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .l_color
                    .push(multiband_color(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.r, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .r_color
                    .push(multiband_color(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.m, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .m_color
                    .push(multiband_color(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.s, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .s_color
                    .push(multiband_color(spectrum.iter().map(|x| x.norm()).collect()));
            }

            // Peak
            // DB
            amp_l += l.abs();
            amp_r += r.abs();
            //IIR
            let iir_l = combined_filter(l, &mut buffer.peak.iir_l);
            let iir_r = combined_filter(r, &mut buffer.peak.iir_r);
            buffer.peak.sum += iir_l * iir_l + iir_r * iir_r;
            buffer.peak.index += 1;
            if buffer.peak.index >= 4800 {
                let peak_buffer = buffer.peak.clone();
                buffer.peak.reset_sum();
                send_data.iir_data.push(peak_buffer.sum / 4800.0);
            }
        }
        // Stereo
        send_data.stereo_data.max = buffer.stereo.max;
        buffer.stereo.max = f32::NEG_INFINITY;

        // DB
        amp_l /= len as f32;
        amp_r /= len as f32;
        send_data.db_data.l = gain_to_db(amp_l);
        send_data.db_data.r = gain_to_db(amp_r);

        tx.send(send_data).unwrap();
    })
}
