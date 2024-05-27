use crate::{setting::*, utils::*};
use crossbeam_channel::Sender;
use egui::Pos2;
use realfft::RealFftPlanner;
use std::sync::{Arc, Mutex};

const SQRT_2: f32 = 1.4142135;

pub fn get_callback(
    tx: Sender<SendData>,
    buffer: Arc<Mutex<AudioSourceBuffer>>,
    setting: Arc<Mutex<Setting>>,
) -> Box<dyn FnMut(Vec<Vec<f32>>) + Send + Sync> {
    Box::new(move |data: Vec<Vec<f32>>| {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("callback");

        let mut buffer = buffer.lock().unwrap();
        let setting = setting.lock().unwrap();
        let waveform_on = setting.sequence[1].contains(&ModuleList::Waveform);
        let peak_on = setting.sequence[1].contains(&ModuleList::Peak);
        let stereo_on = setting.sequence[1].contains(&ModuleList::Stereogram);
        let spectrum_on = setting.sequence[1].contains(&ModuleList::Spectrum);
        let spectrogram_on = setting.sequence[1].contains(&ModuleList::Spectrogram);

        let waveform_block_length = 280;
        let stereo_block_length = 1;
        let mut send_data = SendData::new();
        let len = data[0].len();
        let mut amp_l = 0.0;
        let mut amp_r = 0.0;

        // Loop for each sample
        for i in 0..len {
            let l = data[0][i];
            let r = data[1][i];
            let m = (l + r) / 2.0;
            let s = (l - r) / 2.0;
            let low_l = multiband_low_filter(l, &mut buffer.multiband.low_buf);
            let low_r = multiband_low_filter(r, &mut buffer.multiband.low_buf);
            let low_m = multiband_low_filter(m, &mut buffer.multiband.low_buf);
            let low_s = multiband_low_filter(s, &mut buffer.multiband.low_buf);
            let mid_l = multiband_mid_filter(l, &mut buffer.multiband.mid_buf);
            let mid_r = multiband_mid_filter(r, &mut buffer.multiband.mid_buf);
            let mid_m = multiband_mid_filter(m, &mut buffer.multiband.mid_buf);
            let mid_s = multiband_mid_filter(s, &mut buffer.multiband.mid_buf);
            let high_l = multiband_high_filter(l, &mut buffer.multiband.high_buf);
            let high_r = multiband_high_filter(r, &mut buffer.multiband.high_buf);
            let high_m = multiband_high_filter(m, &mut buffer.multiband.high_buf);
            let high_s = multiband_high_filter(s, &mut buffer.multiband.high_buf);
            buffer.raw.l.push(l);
            buffer.raw.r.push(r);
            buffer.raw.m.push(m);
            buffer.raw.s.push(s);
            buffer.low_raw.l.push(low_l);
            buffer.low_raw.r.push(low_r);
            buffer.low_raw.m.push(low_m);
            buffer.low_raw.s.push(low_s);
            buffer.mid_raw.l.push(mid_l);
            buffer.mid_raw.r.push(mid_r);
            buffer.mid_raw.m.push(mid_m);
            buffer.mid_raw.s.push(mid_s);
            buffer.high_raw.l.push(high_l);
            buffer.high_raw.r.push(high_r);
            buffer.high_raw.m.push(high_m);
            buffer.high_raw.s.push(high_s);

            buffer.fft_1024_index += 1;
            let raw_len = buffer.raw.l.len();

            if waveform_on {
                // Waveform
                buffer.waveform.update_l(l);
                buffer.waveform.update_r(r);
                buffer.waveform.update_m(m);
                buffer.waveform.update_s(s);
                buffer.waveform.index += 1;
                if buffer.waveform.index >= waveform_block_length {
                    let waveform_buffer = buffer.waveform.clone();
                    buffer.waveform.reset();

                    let mut real_planner = RealFftPlanner::<f32>::new();
                    let r2c = real_planner.plan_fft_forward(1024);
                    let mut spectrum = r2c.make_output_vec();

                    let mut waveform_temp_l = vec![0.0; 1024];
                    waveform_temp_l[..waveform_block_length]
                        .copy_from_slice(&buffer.raw.l[raw_len - waveform_block_length..raw_len]);
                    r2c.process(&mut waveform_temp_l, &mut spectrum).unwrap();
                    send_data.waveform_data.l.push(WaveformSendFrame {
                        value: waveform_buffer.l,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_r = vec![0.0; 1024];
                    waveform_temp_r[..waveform_block_length].copy_from_slice(
                        &buffer.raw.r
                            [buffer.raw.r.len() - waveform_block_length..buffer.raw.r.len()],
                    );
                    r2c.process(&mut waveform_temp_r, &mut spectrum).unwrap();
                    send_data.waveform_data.r.push(WaveformSendFrame {
                        value: waveform_buffer.r,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_m = vec![0.0; 1024];
                    waveform_temp_m[..waveform_block_length].copy_from_slice(
                        &buffer.raw.m
                            [buffer.raw.m.len() - waveform_block_length..buffer.raw.m.len()],
                    );
                    r2c.process(&mut waveform_temp_m, &mut spectrum).unwrap();
                    send_data.waveform_data.m.push(WaveformSendFrame {
                        value: waveform_buffer.m,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_s = vec![0.0; 1024];
                    waveform_temp_s[..waveform_block_length].copy_from_slice(
                        &buffer.raw.s
                            [buffer.raw.s.len() - waveform_block_length..buffer.raw.s.len()],
                    );
                    r2c.process(&mut waveform_temp_s, &mut spectrum).unwrap();
                    send_data.waveform_data.s.push(WaveformSendFrame {
                        value: waveform_buffer.s,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });
                }
            }

            if stereo_on {
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
            }

            if peak_on {
                // Peak
                // DB
                amp_l += l.abs();
                amp_r += r.abs();
                //IIR
                let iir_l = lufs_combined_filter(l, &mut buffer.peak.iir_l);
                let iir_r = lufs_combined_filter(r, &mut buffer.peak.iir_r);
                buffer.peak.sum += iir_l * iir_l + iir_r * iir_r;
                buffer.peak.index += 1;
                if buffer.peak.index >= 4800 {
                    let peak_buffer = buffer.peak.clone();
                    buffer.peak.reset_sum();
                    send_data.iir_data.push(peak_buffer.sum / 4800.0);
                }
            }

            if buffer.raw.l.len() >= 4096 {
                buffer.raw.keep_last(2048);
                buffer.low_raw.keep_last(2048);
                buffer.mid_raw.keep_last(2048);
                buffer.high_raw.keep_last(2048);
            }
        }

        // Send data

        if stereo_on {
            // Stereo
            send_data.stereo_data.max = buffer.stereo.max;
            buffer.stereo.max = f32::NEG_INFINITY;
        }

        if peak_on {
            // DB
            amp_l /= len as f32;
            amp_r /= len as f32;
            send_data.db_data.l = gain_to_db(amp_l);
            send_data.db_data.r = gain_to_db(amp_r);
        }

        tx.send(send_data).unwrap();
    })
}
