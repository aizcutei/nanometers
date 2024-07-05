use crate::{setting::*, utils::*};
// use crossbeam_channel::{Receiver, Sender};
use egui::*;
use realfft::RealFftPlanner;
use rustfft::num_complex::{Complex, ComplexFloat};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
const SQRT_2: f32 = 1.4142135;

pub fn get_callback(
    tx_data: Sender<SendData>,
    setting: Arc<Mutex<Setting>>,
    buffer: Arc<Mutex<AudioSourceBuffer>>,
) -> Box<dyn FnMut(Vec<Vec<f32>>) + Send> {
    Box::new(move |data: Vec<Vec<f32>>| {
        #[cfg(feature = "puffin")]
        puffin::profile_scope!("callback");

        let mut buf = buffer.lock().unwrap();
        {
            let setting = setting.lock().unwrap();
            buf.setting = setting.clone();
        }

        let mut send_data = SendData::new();
        let len = data[0].len();

        let waveform_on = buf.setting.meters[1].contains(&MeterList::Waveform);
        let peak_on = buf.setting.meters[1].contains(&MeterList::Peak);
        let vector_on = buf.setting.meters[1].contains(&MeterList::Vectorscope);
        let spectrum_on = buf.setting.meters[1].contains(&MeterList::Spectrum);
        let spectrogram_on = buf.setting.meters[1].contains(&MeterList::Spectrogram);
        let oscilloscope_on = buf.setting.meters[1].contains(&MeterList::Oscilloscope);

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
                if buf.spectrogram.frames.is_empty() {
                    buf.spectrogram
                        .frames
                        .push_back(SpectrogramCalcFrame::new());
                    buf.spectrogram.frames[0].raw_hann.push(m * HANN_2048[0]);
                    buf.spectrogram.frames[0]
                        .raw_hann_t
                        .push(m * HANN_T_2048[0]);
                    buf.spectrogram.frames[0]
                        .raw_hann_dt
                        .push(m * HANN_DT_2048[0]);
                    buf.spectrogram.frames[0].index += 1;
                } else {
                    let frame_number = buf.spectrogram.frames.len();
                    for i in 0..frame_number {
                        let last_index = buf.spectrogram.frames[i].index;
                        buf.spectrogram.frames[i]
                            .raw_hann
                            .push(m * HANN_2048[last_index]);
                        buf.spectrogram.frames[i]
                            .raw_hann_t
                            .push(m * HANN_T_2048[last_index]);
                        buf.spectrogram.frames[i]
                            .raw_hann_dt
                            .push(m * HANN_DT_2048[last_index]);
                        buf.spectrogram.frames[i].index += 1;
                    }
                    if buf.spectrogram.frames[frame_number - 1].index >= 256 {
                        buf.spectrogram
                            .frames
                            .push_back(SpectrogramCalcFrame::new());
                    }
                    if buf.spectrogram.frames[0].index >= spectrogram_fft_size {
                        let mut frame = buf.spectrogram.frames.pop_front().unwrap();
                        let mut real_planner = RealFftPlanner::<f32>::new();
                        let r2c = real_planner.plan_fft_forward(2048);
                        let mut spectrum = r2c.make_output_vec();
                        r2c.process(&mut frame.raw_hann, &mut spectrum).unwrap();
                        let fft_x = spectrum.clone();
                        match buf.setting.spectrogram.mode {
                            SpectrogramMode::Sharp => {
                                let magsqrd: Vec<_> = fft_x.iter().map(|i| i.norm_sqr()).collect();
                                r2c.process(&mut frame.raw_hann_dt, &mut spectrum).unwrap();
                                let fft_xdt = spectrum.clone();
                                r2c.process(&mut frame.raw_hann_t, &mut spectrum).unwrap();
                                let fft_xt = spectrum.clone();
                                for i in 0..1025 {
                                    if magsqrd[i] > 0.0 {
                                        let fc_fix =
                                            -(fft_xdt[i] * fft_x[i].conj()).im() / magsqrd[i];
                                        let fc = fc_fix + FREQFRAME_2048[i];
                                        let tc_fix =
                                            (fft_xt[i] * fft_x[i].conj()).re() / magsqrd[i];
                                        let tc = tc_fix + 0.02133333333; // 1024/48000
                                        let db = 20.0 * (fft_x[i].norm() / 2048.0).log10();
                                        let f = if buf.setting.spectrogram.curve
                                            == SpectrogramCurve::Linear
                                        {
                                            fc / 22000.0
                                        } else {
                                            0.2991878257 * (fc.log10() - 1.0)
                                        };
                                        let t = tc / 0.02133333333;
                                        let c = (db + 70.0) * 255.0 / 70.0
                                            * buf.setting.spectrogram.brightness_boost as f32;
                                        send_data.spectrogram.f.push(f);
                                        send_data.spectrogram.t.push(t);
                                        send_data.spectrogram.i.push(c as u8);
                                    }
                                }
                            }
                            SpectrogramMode::Classic => {
                                for i in 0..1025 {
                                    let intensity = fft_x[i].norm() / 2048.0;
                                    let db = 20.0 * intensity.log10();
                                    let color = if db >= -60.0 {
                                        ((db + 60.0)
                                            * 4.25
                                            * buf.setting.spectrogram.brightness_boost as f32)
                                            as u8
                                    } else {
                                        0
                                    };
                                    send_data.spectrogram.classic_i.push(color);
                                }
                            }
                        }
                    }
                }
            } else {
                if !buf.spectrogram.frames.is_empty() {
                    buf.spectrogram.frames.clear();
                }
            }

            if oscilloscope_on {
                // Oscilloscope
                if buf.setting.oscilloscope.follow_pitch {
                    match buf.setting.oscilloscope.cycle {
                        OscilloscopeCycle::Multi => {
                            if buf.osc.last.is_none() {
                                if m > 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                }
                            } else {
                                let last_times_m = buf.osc.last.unwrap() * m;
                                if last_times_m > 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                } else if last_times_m == 0.0 {
                                    buf.osc.raw.push(m);
                                } else if last_times_m < 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                    buf.osc.even_trun = !buf.osc.even_trun;
                                }
                            }
                            if buf.osc.raw.len() >= 2400 && buf.osc.even_trun {
                                send_data.oscilloscope = OscilloscopeSendData {
                                    len: buf.osc.raw.len(),
                                    data: buf.osc.raw.clone(),
                                };
                                buf.osc.clear();
                            }
                        }
                        OscilloscopeCycle::Single => {
                            if buf.osc.last.is_none() {
                                if m > 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                }
                            } else {
                                let last_times_m = buf.osc.last.unwrap() * m;
                                if last_times_m > 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                } else if last_times_m == 0.0 {
                                    buf.osc.raw.push(m);
                                } else if last_times_m < 0.0 {
                                    buf.osc.raw.push(m);
                                    buf.osc.last = Some(m);
                                    if buf.osc.even_trun {
                                        buf.osc.first_turn = true;
                                    }
                                    buf.osc.even_trun = !buf.osc.even_trun;
                                }
                            }
                            if buf.osc.first_turn && buf.osc.even_trun {
                                send_data.oscilloscope = OscilloscopeSendData {
                                    len: buf.osc.raw.len(),
                                    data: buf.osc.raw.clone(),
                                };
                                buf.osc.clear();
                            }
                        }
                    }
                    // in case of buffer overflow
                    if buf.osc.raw.len() >= 4096 {
                        send_data.oscilloscope = OscilloscopeSendData {
                            len: buf.osc.raw.len(),
                            data: buf.osc.raw.clone(),
                        };
                        buf.osc.clear();
                    }
                } else {
                    buf.osc.raw.push(m);
                    if buf.osc.raw.len() >= 2400 {
                        send_data.oscilloscope = OscilloscopeSendData {
                            len: 2400,
                            data: buf.osc.raw.clone(),
                        };
                        buf.osc.clear();
                    }
                }
            }

            if waveform_on || vector_on {
                let low_l = multiband_low_filter(l, &mut buf.multiband.low_buf.l);
                let low_r = multiband_low_filter(r, &mut buf.multiband.low_buf.r);
                let mid_l = multiband_mid_filter(l, &mut buf.multiband.mid_buf.l);
                let mid_r = multiband_mid_filter(r, &mut buf.multiband.mid_buf.r);
                let high_l = multiband_high_filter(l, &mut buf.multiband.high_buf.l);
                let high_r = multiband_high_filter(r, &mut buf.multiband.high_buf.r);
                let low_m = (low_l + low_r) / 2.0;
                let low_s = (low_l - low_r) / 2.0;
                let mid_m = (mid_l + mid_r) / 2.0;
                let mid_s = (mid_l - mid_r) / 2.0;
                let high_m = (high_l + high_r) / 2.0;
                let high_s = (high_l - high_r) / 2.0;

                if waveform_on {
                    // Waveform
                    buf.waveform.update(l, r, m, s);
                    buf.waveform.update_low(low_l, low_r, low_m, low_s);
                    buf.waveform.update_mid(mid_l, mid_r, mid_m, mid_s);
                    buf.waveform.update_high(high_l, high_r, high_m, high_s);
                    buf.waveform.index += 1;
                    // println!("{},{},{}", low_l, mid_l, high_l);
                    if buf.waveform.index >= waveform_block_length {
                        let waveform_buffer = buf.waveform.clone();
                        buf.waveform.reset();

                        send_data.waveform.l.push(WaveformSendFrame {
                            value: waveform_buffer.l,
                            color: normalize_color(
                                waveform_buffer.low.l,
                                waveform_buffer.mid.l,
                                waveform_buffer.high.l,
                            ),
                        });
                        send_data.waveform.r.push(WaveformSendFrame {
                            value: waveform_buffer.r,
                            color: normalize_color(
                                waveform_buffer.low.r,
                                waveform_buffer.mid.r,
                                waveform_buffer.high.r,
                            ),
                        });
                        send_data.waveform.m.push(WaveformSendFrame {
                            value: waveform_buffer.m,
                            color: normalize_color(
                                waveform_buffer.low.m,
                                waveform_buffer.mid.m,
                                waveform_buffer.high.m,
                            ),
                        });
                        send_data.waveform.s.push(WaveformSendFrame {
                            value: waveform_buffer.s,
                            color: normalize_color(
                                waveform_buffer.low.s,
                                waveform_buffer.mid.s,
                                waveform_buffer.high.s,
                            ),
                        });
                    }
                }

                if vector_on {
                    // Vector
                    buf.vector
                        .update(l, r, low_l, low_r, mid_l, mid_r, high_l, high_r);
                    match buf.setting.vectorscope.mode {
                        VectorscopeMode::Linear => match buf.setting.vectorscope.color {
                            VectorscopeColor::Static => {
                                send_data.vectorscope.r.push(pos2(-SQRT_2 * s, -SQRT_2 * m));
                            }
                            VectorscopeColor::MultiBand => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data
                                        .vectorscope
                                        .r
                                        .push(pos2(-SQRT_2 * low_s, -SQRT_2 * low_m));
                                    send_data
                                        .vectorscope
                                        .g
                                        .push(pos2(-SQRT_2 * mid_s, -SQRT_2 * mid_m));
                                    send_data
                                        .vectorscope
                                        .b
                                        .push(pos2(-SQRT_2 * high_s, -SQRT_2 * high_m));
                                }
                            }
                            VectorscopeColor::RGB => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data.vectorscope.r.push(pos2(-SQRT_2 * s, -SQRT_2 * m));
                                    send_data.vectorscope.c.push(normalize_additive_color(
                                        (low_l * low_l + low_r * low_r).sqrt(),
                                        (mid_l * mid_l + mid_r * mid_r).sqrt(),
                                        (high_l * high_l + high_r * high_r).sqrt(),
                                    ));
                                }
                            }
                        },
                        VectorscopeMode::Lissajous => match buf.setting.vectorscope.color {
                            VectorscopeColor::Static => {
                                send_data.vectorscope.r.push(pos2(l, r));
                            }
                            VectorscopeColor::MultiBand => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data.vectorscope.r.push(pos2(low_l, low_r));
                                    send_data.vectorscope.g.push(pos2(mid_l, mid_r));
                                    send_data.vectorscope.b.push(pos2(high_l, high_r));
                                }
                            }
                            VectorscopeColor::RGB => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data.vectorscope.r.push(pos2(l, r));
                                    send_data.vectorscope.c.push(normalize_additive_color(
                                        (low_l * low_l + low_r * low_r).sqrt(),
                                        (mid_l * mid_l + mid_r * mid_r).sqrt(),
                                        (high_l * high_l + high_r * high_r).sqrt(),
                                    ));
                                }
                            }
                        },
                        VectorscopeMode::Logarithmic => match buf.setting.vectorscope.color {
                            VectorscopeColor::Static => {
                                send_data.vectorscope.r.push(vector_scale_pos(l, r));
                            }
                            VectorscopeColor::MultiBand => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data.vectorscope.r.push(vector_scale_pos(low_l, low_r));
                                    send_data.vectorscope.g.push(vector_scale_pos(mid_l, mid_r));
                                    send_data
                                        .vectorscope
                                        .b
                                        .push(vector_scale_pos(high_l, high_r));
                                }
                            }
                            VectorscopeColor::RGB => {
                                if buf.vector.index >= vector_block_length {
                                    buf.vector.index = 0;
                                    send_data.vectorscope.r.push(vector_scale_pos(l, r));
                                    send_data.vectorscope.c.push(normalize_additive_color(
                                        (low_l * low_l + low_r * low_r).sqrt(),
                                        (mid_l * mid_l + mid_r * mid_r).sqrt(),
                                        (high_l * high_l + high_r * high_r).sqrt(),
                                    ));
                                }
                            }
                        },
                    }
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
        }

        // Send data
        if vector_on {
            // Vector
            send_data.vectorscope.max = buf.vector.max;
            send_data.vectorscope.r_max = buf.vector.r_max;
            send_data.vectorscope.g_max = buf.vector.g_max;
            send_data.vectorscope.b_max = buf.vector.b_max;
            buf.vector.reset();
        }

        if peak_on {
            // DB
            amp_l /= len as f32;
            amp_r /= len as f32;
            send_data.db.l = gain_to_db(amp_l);
            send_data.db.r = gain_to_db(amp_r);
        }

        tx_data.send(send_data).unwrap_or_default();
    })
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
            send_data.spectrum.l = spectrum
                .clone()
                .iter()
                .map(|i| amp_to_db(i, buf.setting.spectrum.low, buf.setting.spectrum.high))
                .collect();
            r2c.process(&mut data.r, &mut spectrum).unwrap();
            send_data.spectrum.r = spectrum
                .clone()
                .iter()
                .map(|i| amp_to_db(i, buf.setting.spectrum.low, buf.setting.spectrum.high))
                .collect();
        }
        SpectrumChannel::MS => {
            data.m.extend_from_slice(&[0.0; 2048]);
            data.s.extend_from_slice(&[0.0; 2048]);
            r2c.process(&mut data.m, &mut spectrum).unwrap();
            send_data.spectrum.l = spectrum
                .clone()
                .iter()
                .map(|i| amp_to_db(i, buf.setting.spectrum.low, buf.setting.spectrum.high))
                .collect();
            r2c.process(&mut data.s, &mut spectrum).unwrap();
            send_data.spectrum.r = spectrum
                .clone()
                .iter()
                .map(|i| amp_to_db(i, buf.setting.spectrum.low, buf.setting.spectrum.high))
                .collect();
        }
    }
}

fn amp_to_db(amp: &Complex<f32>, low: f32, high: f32) -> f32 {
    ((amp.norm() / 2048.0).log10() * 20.0 - low - high + 20.0) / (-low - high + 20.0)
}

fn vector_scale_pos(l: f32, r: f32) -> Pos2 {
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
    pos2(
        0.7071067812 * (log_y - log_x),
        -0.7071067812 * (log_x + log_y),
    )
}
