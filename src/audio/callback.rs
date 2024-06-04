use crate::{setting::*, utils::*};
use crossbeam_channel::{Receiver, Sender};
use egui::{Color32, Pos2};
use realfft::RealFftPlanner;
use rustfft::num_complex::{Complex, ComplexFloat};
use std::sync::{Arc, Mutex};

const SQRT_2: f32 = 1.4142135;

pub fn get_callback(
    tx_data: Sender<SendData>,
    rx_setting: Receiver<Setting>,
    buffer: Arc<Mutex<AudioSourceBuffer>>,
) -> Box<dyn FnMut(Vec<Vec<f32>>) + Send + Sync> {
    Box::new(move |data: Vec<Vec<f32>>| {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("callback");

        let mut buf = buffer.lock().unwrap();
        rx_setting.try_iter().for_each(|s| buf.setting = s);
        let mut send_data = SendData::new();
        let len = data[0].len();

        let waveform_on = buf.setting.sequence[1].contains(&ModuleList::Waveform);
        let peak_on = buf.setting.sequence[1].contains(&ModuleList::Peak);
        let vector_on = buf.setting.sequence[1].contains(&ModuleList::Vectorscope);
        let spectrum_on = buf.setting.sequence[1].contains(&ModuleList::Spectrum);
        let spectrogram_on = buf.setting.sequence[1].contains(&ModuleList::Spectrogram);
        let oscilloscope_on = buf.setting.sequence[1].contains(&ModuleList::Oscilloscope);

        let waveform_block_length = 280;
        let vector_block_length = 1;
        let spectrogram_fft_size = 2048;
        let mut amp_l = 0.0;
        let mut amp_r = 0.0;

        // Loop for each sample
        for i in 0..len {
            let l = data[0][i];
            let r = data[1][i];
            let m = (l + r) / 2.0;
            let s = (l - r) / 2.0;
            buf.raw.l.push(l);
            buf.raw.r.push(r);
            buf.raw.m.push(m);
            buf.raw.s.push(s);

            let raw_len = buf.raw.l.len();

            if spectrum_on {
                if buf.spectrum.ab {
                    // Update Buffer
                    let spectrum_index = buf.spectrum.a.l.len();
                    if spectrum_index >= 1024 {
                        buf.spectrum.b.l.push(l * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.b.r.push(r * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.b.m.push(m * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.b.s.push(s * HANN_2048[spectrum_index - 1024]);
                    }
                    buf.spectrum.a.l.push(l * HANN_2048[spectrum_index]);
                    buf.spectrum.a.r.push(r * HANN_2048[spectrum_index]);
                    buf.spectrum.a.m.push(m * HANN_2048[spectrum_index]);
                    buf.spectrum.a.s.push(s * HANN_2048[spectrum_index]);
                    // Calculate FFT
                    if buf.spectrum.a.l.len() >= 2048 {
                        let mut a_data = buf.spectrum.a.clone();
                        process_spectrum(&mut buf, &mut send_data, &mut a_data);
                        buf.spectrum.a.clear();
                    }
                } else {
                    let spectrum_index = buf.spectrum.b.l.len();
                    if spectrum_index >= 1024 {
                        buf.spectrum.a.l.push(l * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.a.r.push(r * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.a.m.push(m * HANN_2048[spectrum_index - 1024]);
                        buf.spectrum.a.s.push(s * HANN_2048[spectrum_index - 1024]);
                    }
                    buf.spectrum.b.l.push(l * HANN_2048[spectrum_index]);
                    buf.spectrum.b.r.push(r * HANN_2048[spectrum_index]);
                    buf.spectrum.b.m.push(m * HANN_2048[spectrum_index]);
                    buf.spectrum.b.s.push(s * HANN_2048[spectrum_index]);
                    if buf.spectrum.b.l.len() >= 2048 {
                        let mut b_data = buf.spectrum.b.clone();
                        process_spectrum(&mut buf, &mut send_data, &mut b_data);
                        buf.spectrum.b.clear();
                    }
                }
            } else {
                buf.spectrum.a.clear();
                buf.spectrum.b.clear();
            }

            if spectrogram_on {
                if buf.spectrogram.ab {
                    let spectrogram_index = buf.spectrogram.a.index.clone();
                    if spectrogram_index >= spectrogram_fft_size / 2 {
                        updata_spectrogram_window(
                            &mut buf.spectrogram.b,
                            spectrogram_index - spectrogram_fft_size / 2,
                            m,
                        );
                    }
                    updata_spectrogram_window(&mut buf.spectrogram.a, spectrogram_index, m);
                    if buf.spectrogram.a.index >= spectrogram_fft_size {
                        let mut spectrum_buffer = buf.spectrogram.clone();
                        process_spectrogram(
                            &mut buf,
                            &mut send_data,
                            &mut spectrum_buffer.a.raw_hann,
                            &mut spectrum_buffer.a.raw_hann_dt,
                            &mut spectrum_buffer.a.raw_hann_t,
                        );
                        buf.spectrogram.a.reset();
                    }
                } else {
                    let spectrogram_index = buf.spectrogram.b.index.clone();
                    if spectrogram_index >= spectrogram_fft_size / 2 {
                        updata_spectrogram_window(
                            &mut buf.spectrogram.a,
                            spectrogram_index - spectrogram_fft_size / 2,
                            m,
                        );
                    }
                    updata_spectrogram_window(&mut buf.spectrogram.b, spectrogram_index, m);
                    if buf.spectrogram.b.index >= spectrogram_fft_size {
                        let mut spectrum_buffer = buf.spectrogram.clone();
                        process_spectrogram(
                            &mut buf,
                            &mut send_data,
                            &mut spectrum_buffer.b.raw_hann,
                            &mut spectrum_buffer.b.raw_hann_dt,
                            &mut spectrum_buffer.b.raw_hann_t,
                        );
                        buf.spectrogram.b.reset();
                    }
                }
            } else {
                buf.spectrogram.a.reset();
                buf.spectrogram.b.reset();
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

            if vector_on {
                // Stereo
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
                buf.stereo.update(l, r);
                if buf.stereo.index >= vector_block_length {
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

        if vector_on {
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

        tx_data.send(send_data).unwrap();
    })
}

fn process_spectrogram(
    buf: &mut AudioSourceBuffer,
    send_data: &mut SendData,
    raw_hann: &mut [f32],
    raw_hann_dt: &mut [f32],
    raw_hann_t: &mut [f32],
) {
    let resolution = 2048;
    let speed = 4;
    buf.spectrogram.image.drain(0..resolution * speed);
    buf.spectrogram
        .image
        .extend(vec![Color32::TRANSPARENT; resolution * speed]);
    let mut real_planner = RealFftPlanner::<f32>::new();
    let r2c = real_planner.plan_fft_forward(2048);
    let mut spectrum = r2c.make_output_vec();
    r2c.process(raw_hann, &mut spectrum).unwrap();
    let fft_x = spectrum.clone();
    let magsqrd: Vec<_> = fft_x.iter().map(|i| i.norm_sqr()).collect();
    r2c.process(raw_hann_dt, &mut spectrum).unwrap();
    let fft_xdt = spectrum.clone();
    r2c.process(raw_hann_t, &mut spectrum).unwrap();
    let fft_xt = spectrum.clone();
    for i in 0..1025 {
        let fc_temp = (-(fft_xdt[i] * fft_x[i].conj()).im() / magsqrd[i]) + FREQFRAME_2048[i];
        let tc_temp = (fft_xt[i] * fft_x[i].conj()).re() / magsqrd[i];

        if fc_temp > 0.0 && fc_temp < 22000.0 {
            let image_x = if buf.setting.spectrogram.curve == SpectrogramCurve::Linear {
                fc_temp.floor() / 22000.0 * resolution as f32
            } else {
                0.2991878257 * (fc_temp.log10() - 1.0) * resolution as f32
            };
            let image_y = 1920.0 + tc_temp * 46.875 * speed as f32;

            let o_x_weight = image_x - image_x.floor();
            let o_y_weight = image_y - image_y.floor();
            let o_x = image_x.floor() as usize;
            let o_y = image_y.floor() as usize;
            let o_00_weight = o_x_weight * o_y_weight;
            let o_01_weight = o_x_weight * (1.0 - o_y_weight);
            let o_10_weight = (1.0 - o_x_weight) * o_y_weight;
            let o_11_weight = (1.0 - o_x_weight) * (1.0 - o_y_weight);
            let o_00_index = o_x + o_y * resolution;
            let o_01_index = o_x + (o_y + 1) * resolution;
            let o_10_index = o_x + 1 + o_y * resolution;
            let o_11_index = o_x + 1 + (o_y + 1) * resolution;

            let o_00_c = buf.spectrogram.image[o_00_index].a();
            let o_01_c = buf.spectrogram.image[o_01_index].a();
            let o_10_c = buf.spectrogram.image[o_10_index].a();
            let o_11_c = buf.spectrogram.image[o_11_index].a();

            let r = buf.setting.theme.main.r();
            let g = buf.setting.theme.main.g();
            let b = buf.setting.theme.main.b();
            let boost = buf.setting.spectrogram.brightness_boost as f32;

            buf.spectrogram.image[o_00_index] = Color32::from_rgba_unmultiplied(
                r,
                g,
                b,
                o_00_c.wrapping_add((boost * 255.0 * fft_x[i].norm() * o_00_weight) as u8),
            );
            buf.spectrogram.image[o_01_index] = Color32::from_rgba_unmultiplied(
                r,
                g,
                b,
                o_01_c.wrapping_add((boost * 255.0 * fft_x[i].norm() * o_01_weight) as u8),
            );
            buf.spectrogram.image[o_10_index] = Color32::from_rgba_unmultiplied(
                r,
                g,
                b,
                o_10_c.wrapping_add((boost * 255.0 * fft_x[i].norm() * o_10_weight) as u8),
            );
            buf.spectrogram.image[o_11_index] = Color32::from_rgba_unmultiplied(
                r,
                g,
                b,
                o_11_c.wrapping_add((boost * 255.0 * fft_x[i].norm() * o_11_weight) as u8),
            );
        }
    }
    send_data.spectrogram_image = buf.spectrogram.image[0..1920 * resolution].to_owned();
    buf.spectrogram.ab = !buf.spectrogram.ab;
}

fn process_spectrum(buf: &mut AudioSourceBuffer, send_data: &mut SendData, data: &mut RawData) {
    let mut real_planner = RealFftPlanner::<f32>::new();
    let r2c = real_planner.plan_fft_forward(4096);
    let mut spectrum = r2c.make_output_vec();
    match buf.setting.spectrum.channel {
        SpectrumChannel::LR => {
            data.l.extend_from_slice(&[0.0; 2048]);
            data.r.extend_from_slice(&[0.0; 2048]);
            r2c.process(&mut data.l, &mut spectrum).unwrap();
            send_data.spectrum.l = spectrum.clone().iter().map(|i| amp_to_db(i)).collect();
            r2c.process(&mut data.r, &mut spectrum).unwrap();
            send_data.spectrum.r = spectrum.clone().iter().map(|i| amp_to_db(i)).collect();
        }
        SpectrumChannel::MS => {
            data.m.extend_from_slice(&[0.0; 2048]);
            data.s.extend_from_slice(&[0.0; 2048]);
            r2c.process(&mut data.m, &mut spectrum).unwrap();
            send_data.spectrum.l = spectrum.clone().iter().map(|i| amp_to_db(i)).collect();
            r2c.process(&mut data.s, &mut spectrum).unwrap();
            send_data.spectrum.r = spectrum.clone().iter().map(|i| amp_to_db(i)).collect();
        }
    }
}

fn amp_to_db(amp: &Complex<f32>) -> f32 {
    ((amp.norm() / 2048.0).log10() * 20.0 + 150.0) / 150.0
}
