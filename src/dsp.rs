use std::sync::{Arc, Mutex};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use spectrum_analyzer::scaling::divide_by_N_sqrt;
use crate::constants::SAMPLE_RATE;
use crate::settings::{FrequenciesValues, SamplesWindow, Settings};

pub fn process_audio_data(
    data: &[f32],
    frequency_levels: &Arc<Mutex<FrequenciesValues>>,
    settings: &Arc<Settings>,
    samples_window: &Arc<Mutex<SamplesWindow>>,
    df: f32,
) {
    // 1.  Move samples into the rolling window
    {
        let mut win = samples_window.lock().unwrap();
        win.add_samples(data);
        if win.samples.lock().unwrap().len() < settings.fft_size {
            return;                       // not enough for one FFT yet
        }
    }

    // 2.  FFT → linear magnitude spectrum (already √N-normalised)
    let spec = samples_fft_to_spectrum(
        &samples_window.lock().unwrap().samples.lock().unwrap(),
        SAMPLE_RATE,
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
    ).expect("FFT");

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
        frequency_levels.lock().unwrap()[i].add_sample(v);  // smooth between frames
    }
}

/// Exponential-law weighting.
/// alpha=0.0  → flat,   alpha>0 → boost highs,   alpha<0 → boost lows.
/// Typical values: alpha = 0.35 … 0.55 gives a gentle but audible lift of everything above ~1 kHz.
pub fn weight(freq_hz: f32, alpha: f32) -> f32 {
    let F_MIN: f32 = 0.0; // min frequency
    let F_MAX: f32 = SAMPLE_RATE as f32 / 2.0; // Nyquist frequency

    // normalised [0,1] then exponential
    let x = ((freq_hz - F_MIN) / (F_MAX - F_MIN)).clamp(0.0, 1.0);
    x.powf(alpha)
}

/// How many neighbouring bins to average on *each* side of the centre bin.
/// 1 ≈ 3-bin window, 2 ≈ 5-bin window, etc.
pub fn half_window_bins(freq_hz: f32, df: f32) -> usize {
    // keep ~15 % fractional bandwidth (tweak to taste)
    let bw_hz = 0.15 * freq_hz;
    ((bw_hz / df).round() as isize).max(1) as usize
}