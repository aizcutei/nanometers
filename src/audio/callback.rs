use realfft::RealFftPlanner;

use std::sync::{Arc, Mutex};

use crate::utils::*;
use crossbeam_channel::Sender;
// use std::collections::VecDeque;
// use std::sync::{Arc, Mutex};

pub fn get_callback(
    tx: Sender<SendData>,
    buffer: Arc<Mutex<AudioSourceBuffer>>,
) -> Box<dyn FnMut(Vec<Vec<f32>>) + Send + Sync> {
    Box::new(move |data: Vec<Vec<f32>>| {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("callback");

        let mut buffer = buffer.lock().unwrap();
        let block_length = 256;

        let mut send_data = SendData::new();
        let len = data[0].len();
        let mut amp_l = 0.0;
        let mut amp_r = 0.0;
        for i in 0..len {
            // Waveform
            buffer.waveform.update_l(data[0][i]);
            buffer.waveform.update_r(data[1][i]);
            buffer.waveform.update_m((data[0][i] + data[1][i]) / 2.0);
            buffer.waveform.update_s((data[0][i] - data[1][i]) / 2.0);
            buffer.waveform.index += 1;
            if buffer.waveform.index >= block_length {
                let mut waveform_buffer = buffer.waveform.clone();
                buffer.waveform.reset();
                send_data.waveform_data.l.push(waveform_buffer.l);
                send_data.waveform_data.r.push(waveform_buffer.r);
                send_data.waveform_data.m.push(waveform_buffer.m);
                send_data.waveform_data.s.push(waveform_buffer.s);

                let mut real_planner = RealFftPlanner::<f32>::new();
                let r2c = real_planner.plan_fft_forward(block_length);
                let mut spectrum = r2c.make_output_vec();
                r2c.process(&mut waveform_buffer.raw.l, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .l_freq
                    .push(max_index(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.r, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .r_freq
                    .push(max_index(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.m, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .m_freq
                    .push(max_index(spectrum.iter().map(|x| x.norm()).collect()));
                r2c.process(&mut waveform_buffer.raw.s, &mut spectrum)
                    .unwrap();
                send_data
                    .waveform_data
                    .s_freq
                    .push(max_index(spectrum.iter().map(|x| x.norm()).collect()));
            }

            // Peak
            // DB
            amp_l += data[0][i].abs();
            amp_r += data[1][i].abs();
            //IIR
            let iir_l = combined_filter(data[0][i], &mut buffer.peak.iir_l);
            let iir_r = combined_filter(data[1][i], &mut buffer.peak.iir_r);
            buffer.peak.sum += iir_l * iir_l + iir_r * iir_r;
            buffer.peak.index += 1;
            if buffer.peak.index >= 4800 {
                let peak_buffer = buffer.peak.clone();
                buffer.peak.reset_sum();
                send_data.iir_data.push(peak_buffer.sum / 4800.0);
            }
        }

        //DB
        amp_l /= len as f32;
        amp_r /= len as f32;
        send_data.db_data.l = gain_to_db(amp_l);
        send_data.db_data.r = gain_to_db(amp_r);

        tx.send(send_data).unwrap();
    })
}

fn max_index(data: Vec<f32>) -> usize {
    data.iter()
        .enumerate()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0)
}
