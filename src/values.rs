use std::sync::{Arc, Mutex};
use crate::constants::FFT_SIZE;
use crate::settings::{Settings};

#[derive(Debug, Clone)]
pub struct StateValues
{
    pub frequencies: FrequenciesValues,
    pub samples_window: SamplesWindow,
}

impl StateValues {

    pub fn new(settings: Arc<Mutex<Settings>>) -> Self {

        let mut result = StateValues {
            frequencies : Vec::new(),
            samples_window: SamplesWindow::new(1024*8),
        };

        result.update_settings(settings);

        result
    }

    pub fn update_settings(&mut self, settings: Arc<Mutex<Settings>>) {

        let smooth_size = settings.lock().unwrap().smooth_size;

        let nb_frequencies = settings.lock().unwrap().frequencies.len();

        self.frequencies.reserve(nb_frequencies);
        for _ in 0..nb_frequencies {
            self.frequencies.push(SamplesWindow::new(100));
        }
    }
}

#[derive(Debug, Clone)]
pub struct SamplesWindow
{
    pub samples: Arc<Mutex<Vec<f32>>>,
    max_size: usize,
}

impl SamplesWindow {
    pub fn new(max_size: usize) -> Self {
        SamplesWindow {
            samples: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            max_size,
        }
    }

    pub fn add_sample(&mut self, sample: f32) {
        let mut samples = self.samples.lock().unwrap();
        if samples.len() >= self.max_size {
            samples.remove(0);
        }
        samples.push(sample);
    }

    pub fn add_samples(&mut self, samples: &[f32]) {
        for sample in samples {
            self.add_sample(*sample);
        }
    }

    pub fn average(&self, size: usize) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            0.0
        } else {
            // take size items from the end of the vector
            let sum: f32 = samples.iter().rev().take(size).rev().sum();
            sum / size as f32
        }
    }

    pub fn max(&self, size: usize) -> f32 {
        let samples = self.samples.lock().unwrap();
        if samples.is_empty() {
            0.0
        } else {
            *samples.iter().take(size).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
        }
    }
}

pub type FrequenciesValues = Vec<SamplesWindow>;