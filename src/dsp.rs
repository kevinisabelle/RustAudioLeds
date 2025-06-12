use std::sync::{Arc, Mutex};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use crate::constants::SAMPLE_RATE;
use crate::settings::{Settings};
use crate::values::{FrequenciesValues, SamplesWindow, StateValues};

pub fn process_audio_data(
    data: &[f32],
    state_values: &Arc<Mutex<StateValues>>,
    settings: &Settings
) {
    let df = settings.cached_df; // frequency bin width

    // 1.  Move samples into the rolling window
    {
        state_values.lock().unwrap().samples_window.add_samples(data);
        if state_values.lock().unwrap().samples_window.samples.lock().unwrap().len() < settings.fft_size as usize {
            println!("Not enough samples for FFT: {} < {}", state_values.lock().unwrap().samples_window.samples.lock().unwrap().len(), settings.fft_size);
            return;                       // not enough for one FFT yet
        }
    }

    // println!("Processing {} samples for FFT", state_values.lock().unwrap().samples_window.samples.lock().unwrap().len());

    // Take fft_size samples from the end of the rolling window
    let samples_window = state_values.lock().unwrap().samples_window.samples.lock().unwrap().
        iter()
        .rev()
        .take(settings.fft_size)
        .cloned()
        .rev()
        .collect::<Vec<f32>>();

    // 2.  FFT → linear magnitude spectrum (already √N-normalised)
    let spec = samples_fft_to_spectrum(
        &samples_window,
        SAMPLE_RATE,
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    ).expect("FFT failed – check sample count");

    for (i, &f_cfg) in settings.frequencies.iter().enumerate() {
        let idx_c = (f_cfg / df).round() as isize;          // centre bin
        let r     = half_window_bins(f_cfg, df);            // window radius

        let mut acc = 0.0;
        let mut n   = 0;
        for off in -(r as isize)..=(r as isize) {
            let idx = idx_c + off;
            if idx >= 0 && (idx as usize) < spec.data().len() {
                acc += spec.data()[idx as usize].1.val();
                n   += 1;
            }
        }
        let mut v = if n > 0 { acc / n as f32 } else { 0.0 };
        v *= weight(f_cfg, settings.skew);                           // high-freq boost
        state_values.lock().unwrap().frequencies[i].add_sample(v);  // smooth between frames
    }
}

/// Exponential-law weighting.
/// alpha=0.0  → flat,   alpha>0 → boost highs,   alpha<0 → boost lows.
/// Typical values: alpha = 0.35 … 0.55 gives a gentle but audible lift of everything above ~1 kHz.
pub fn weight(freq_hz: f32, alpha: f32) -> f32 {
    let f_min: f32 = 0.0; // min frequency
    let f_max: f32 = SAMPLE_RATE as f32 / 2.0; // Nyquist frequency

    // normalised [0,1] then exponential
    let x = ((freq_hz - f_min) / (f_max - f_min)).clamp(0.0, 1.0);
    x.powf(alpha)
}

/// How many neighbouring bins to average on *each* side of the centre bin.
/// 1 ≈ 3-bin window, 2 ≈ 5-bin window, etc.
pub fn half_window_bins(freq_hz: f32, df: f32) -> usize {
    // keep ~15 % fractional bandwidth (tweak to taste)
    let bw_hz = 0.15 * freq_hz;
    ((bw_hz / df).round() as isize).max(1) as usize
}