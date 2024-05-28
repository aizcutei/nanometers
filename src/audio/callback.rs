use crate::{setting::*, utils::*};
use crossbeam_channel::Sender;
use egui::Pos2;
use realfft::RealFftPlanner;
use rustfft::num_complex::ComplexFloat;
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

        let mut buf = buffer.lock().unwrap();
        let setting = setting.lock().unwrap();
        let waveform_on = setting.sequence[1].contains(&ModuleList::Waveform);
        let peak_on = setting.sequence[1].contains(&ModuleList::Peak);
        let stereo_on = setting.sequence[1].contains(&ModuleList::Vectorscope);
        let spectrum_on = setting.sequence[1].contains(&ModuleList::Spectrum);
        let spectrogram_on = setting.sequence[1].contains(&ModuleList::Spectrogram);
        let oscilloscope_on = setting.sequence[1].contains(&ModuleList::Oscilloscope);

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
            let low_l = multiband_low_filter(l, &mut buf.multiband.low_buf);
            let low_r = multiband_low_filter(r, &mut buf.multiband.low_buf);
            let low_m = multiband_low_filter(m, &mut buf.multiband.low_buf);
            let low_s = multiband_low_filter(s, &mut buf.multiband.low_buf);
            let mid_l = multiband_mid_filter(l, &mut buf.multiband.mid_buf);
            let mid_r = multiband_mid_filter(r, &mut buf.multiband.mid_buf);
            let mid_m = multiband_mid_filter(m, &mut buf.multiband.mid_buf);
            let mid_s = multiband_mid_filter(s, &mut buf.multiband.mid_buf);
            let high_l = multiband_high_filter(l, &mut buf.multiband.high_buf);
            let high_r = multiband_high_filter(r, &mut buf.multiband.high_buf);
            let high_m = multiband_high_filter(m, &mut buf.multiband.high_buf);
            let high_s = multiband_high_filter(s, &mut buf.multiband.high_buf);
            buf.raw.l.push(l);
            buf.raw.r.push(r);
            buf.raw.m.push(m);
            buf.raw.s.push(s);
            buf.low_raw.l.push(low_l);
            buf.low_raw.r.push(low_r);
            buf.low_raw.m.push(low_m);
            buf.low_raw.s.push(low_s);
            buf.mid_raw.l.push(mid_l);
            buf.mid_raw.r.push(mid_r);
            buf.mid_raw.m.push(mid_m);
            buf.mid_raw.s.push(mid_s);
            buf.high_raw.l.push(high_l);
            buf.high_raw.r.push(high_r);
            buf.high_raw.m.push(high_m);
            buf.high_raw.s.push(high_s);

            let spectrogram_index = buf.spectrogram.index.clone();
            buf.spectrogram
                .raw_hann
                .push(m * HANN_2048[spectrogram_index]);
            buf.spectrogram
                .raw_hann_dt
                .push(m * HANN_DT_2048[spectrogram_index]);
            buf.spectrogram
                .raw_hann_t
                .push(m * HANN_T_2048[spectrogram_index]);
            buf.spectrogram.index += 1;
            let raw_len = buf.raw.l.len();

            if buf.spectrogram.index >= 2048 {
                if spectrogram_on {
                    let mut spectrum_buffer = buf.spectrogram.clone();

                    let mut real_planner = RealFftPlanner::<f32>::new();
                    let r2c = real_planner.plan_fft_forward(2048);
                    let mut spectrum = r2c.make_output_vec();
                    r2c.process(&mut spectrum_buffer.raw_hann, &mut spectrum)
                        .unwrap();
                    let x = spectrum.clone();
                    let magsqrd: Vec<f32> = x.iter().map(|i| i.norm_sqr()).collect();
                    let magsqrd_max = magsqrd
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(&0.0)
                        .sqrt();
                    r2c.process(&mut spectrum_buffer.raw_hann_dt, &mut spectrum)
                        .unwrap();
                    let xdt = spectrum.clone();
                    r2c.process(&mut spectrum_buffer.raw_hann_t, &mut spectrum)
                        .unwrap();
                    let xt = spectrum.clone();

                    let mut forigin = vec![];
                    let mut fcorrect = vec![];
                    let mut tcorrect = vec![];
                    let mut ccorrect = vec![];
                    for i in 0..1025 {
                        if magsqrd[i] > 0.0 {
                            let fc_temp =
                                (-(xdt[i] * x[i].conj()).im() / magsqrd[i]) + FREQFRAME_2048[i];
                            fcorrect.push(if fc_temp > 0.0 && fc_temp < 24000.0 {
                                fc_temp
                            } else {
                                0.0
                            });
                            tcorrect.push((xt[i] * x[i].conj()).re() / magsqrd[i]);
                            let c_temp = (x[i].norm() / magsqrd_max).log10() * 20.0;
                            ccorrect.push(
                                if fc_temp > 0.0 && fc_temp < 24000.0 && c_temp > -60.0 {
                                    c_temp + 60.0
                                } else {
                                    0.0
                                },
                            );
                        } else {
                            fcorrect.push(0.0);
                            tcorrect.push(0.0);
                            ccorrect.push(0.0);
                        }
                        forigin.push(magsqrd[i].sqrt());
                    }

                    send_data.spectrogram.push(SpectrogramFrame {
                        f: forigin,
                        fc: fcorrect,
                        tc: tcorrect,
                        cc: ccorrect,
                    });
                }
                buf.spectrogram.reset();
            }

            if oscilloscope_on {
                // Oscilloscope
                if raw_len >= 2400 {
                    send_data.oscilloscope = OscilloscopeSendData {
                        len: 2400,
                        data: buf.raw.m[raw_len - 2400..raw_len].to_vec(),
                    };
                }
            }

            if waveform_on {
                // Waveform
                buf.waveform.update_l(l);
                buf.waveform.update_r(r);
                buf.waveform.update_m(m);
                buf.waveform.update_s(s);
                buf.waveform.index += 1;
                if buf.waveform.index >= waveform_block_length {
                    let waveform_buffer = buf.waveform.clone();
                    buf.waveform.reset();

                    let mut real_planner = RealFftPlanner::<f32>::new();
                    let r2c = real_planner.plan_fft_forward(1024);
                    let mut spectrum = r2c.make_output_vec();

                    let mut waveform_temp_l = vec![0.0; 1024];
                    waveform_temp_l[..waveform_block_length]
                        .copy_from_slice(&buf.raw.l[raw_len - waveform_block_length..raw_len]);
                    r2c.process(&mut waveform_temp_l, &mut spectrum).unwrap();
                    send_data.waveform.l.push(WaveformSendFrame {
                        value: waveform_buffer.l,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_r = vec![0.0; 1024];
                    waveform_temp_r[..waveform_block_length].copy_from_slice(
                        &buf.raw.r[buf.raw.r.len() - waveform_block_length..buf.raw.r.len()],
                    );
                    r2c.process(&mut waveform_temp_r, &mut spectrum).unwrap();
                    send_data.waveform.r.push(WaveformSendFrame {
                        value: waveform_buffer.r,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_m = vec![0.0; 1024];
                    waveform_temp_m[..waveform_block_length].copy_from_slice(
                        &buf.raw.m[buf.raw.m.len() - waveform_block_length..buf.raw.m.len()],
                    );
                    r2c.process(&mut waveform_temp_m, &mut spectrum).unwrap();
                    send_data.waveform.m.push(WaveformSendFrame {
                        value: waveform_buffer.m,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });

                    let mut waveform_temp_s = vec![0.0; 1024];
                    waveform_temp_s[..waveform_block_length].copy_from_slice(
                        &buf.raw.s[buf.raw.s.len() - waveform_block_length..buf.raw.s.len()],
                    );
                    r2c.process(&mut waveform_temp_s, &mut spectrum).unwrap();
                    send_data.waveform.s.push(WaveformSendFrame {
                        value: waveform_buffer.s,
                        color: multiband_color(spectrum.iter().map(|x| x.norm()).collect()),
                    });
                }
            }

            if stereo_on {
                // Stereo
                buf.stereo.update(l, r);
                if buf.stereo.index >= stereo_block_length {
                    buf.stereo.index = 0;
                    send_data.vectorscope.lissa.push(Pos2::new(l, r));
                    send_data
                        .vectorscope
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
                    send_data.vectorscope.log.push(Pos2::new(
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
                let iir_l = lufs_combined_filter(l, &mut buf.peak.iir_l);
                let iir_r = lufs_combined_filter(r, &mut buf.peak.iir_r);
                buf.peak.sum += iir_l * iir_l + iir_r * iir_r;
                buf.peak.index += 1;
                if buf.peak.index >= 4800 {
                    let peak_buffer = buf.peak.clone();
                    buf.peak.reset_sum();
                    send_data.iir.push(peak_buffer.sum / 4800.0);
                }
            }

            if raw_len >= 8192 {
                buf.raw.keep_last(4096);
                buf.low_raw.keep_last(4096);
                buf.mid_raw.keep_last(4096);
                buf.high_raw.keep_last(4096);
            }
        }

        // Send data

        if stereo_on {
            // Stereo
            send_data.vectorscope.max = buf.stereo.max;
            buf.stereo.max = f32::NEG_INFINITY;
        }

        if peak_on {
            // DB
            amp_l /= len as f32;
            amp_r /= len as f32;
            send_data.db.l = gain_to_db(amp_l);
            send_data.db.r = gain_to_db(amp_r);
        }

        tx.send(send_data).unwrap();
    })
}
