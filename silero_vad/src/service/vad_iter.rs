use crate::service::silero::SileroSession;
use crate::utils::utils::{FrameStamp, TimeStamp, VadParams};
use lazy_static::lazy_static;

lazy_static! {
    static ref DEBUG_SPEECH_PROB: bool = std::env::var("DEBUG_SPEECH_PROB").is_ok_and(|s| s == "true");
}

#[derive(Debug)]
pub struct VadIter {
    silero: SileroSession,
    params: Params,
    state: State,
}

impl VadIter {
    pub fn new(silero: SileroSession, params: VadParams) -> Self {
        Self {
            silero,
            state: State::new(params.sample_rate.clone()),
            params: Params::from(params),
        }
    }

    pub fn process(&mut self, samples: &[i16]) -> Result<(), ort::Error> {
        self.reset_states();
        for audio_frame in samples.chunks_exact(self.params.frame_size_samples) {
            let speech_prob: f32 = self.silero.calc_level(audio_frame)?;
            self.state.update(&self.params, speech_prob);
        }
        self.state.check_for_last_speech(samples.len());
        Ok(())
    }

    pub fn speeches(&self) -> Vec<TimeStamp> {
        self.state.speeches.iter().fold(Vec::new(), |mut acc, speech| {
            if let Some(last) = acc.last_mut() {
                if last.end == speech.start {
                    last.end = speech.end;
                    return acc;
                }
            }
            acc.push(speech.clone());
            acc
        })
    }
    fn reset_states(&mut self) {
        self.silero.reset();
        self.state = State::new(self.params.sample_rate)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct Params {
    frame_size: usize,
    threshold: f32,
    min_silence_duration_ms: usize,
    speech_pad_ms: usize,
    min_speech_duration_ms: usize,
    max_speech_duration_s: f32,
    sample_rate: usize,
    sr_per_ms: usize,
    frame_size_samples: usize,
    min_speech_samples: usize,
    speech_pad_samples: usize,
    max_speech_samples: f32,
    min_silence_samples: usize,
    min_silence_samples_at_max_speech: usize,
}

impl From<VadParams> for Params {
    fn from(value: VadParams) -> Self {
        let frame_size = value.frame_size;
        let threshold = value.threshold;
        let min_silence_duration_ms = value.min_silence_duration_ms;
        let speech_pad_ms = value.speech_pad_ms;
        let min_speech_duration_ms = value.min_speech_duration_ms;
        let max_speech_duration_s = value.max_speech_duration_s;
        let sample_rate = value.sample_rate;
        let sr_per_ms = sample_rate / 1000;
        let frame_size_samples = frame_size * sr_per_ms;
        let min_speech_samples = sr_per_ms * min_speech_duration_ms;
        let speech_pad_samples = sr_per_ms * speech_pad_ms;
        let max_speech_samples =
            sample_rate as f32 * max_speech_duration_s - frame_size_samples as f32 - 2.0 * speech_pad_samples as f32;
        let min_silence_samples = sr_per_ms * min_silence_duration_ms;
        let min_silence_samples_at_max_speech = sr_per_ms * 98;
        Self {
            frame_size,
            threshold,
            min_silence_duration_ms,
            speech_pad_ms,
            min_speech_duration_ms,
            max_speech_duration_s,
            sample_rate,
            sr_per_ms,
            frame_size_samples,
            min_speech_samples,
            speech_pad_samples,
            max_speech_samples,
            min_silence_samples,
            min_silence_samples_at_max_speech,
        }
    }
}

#[derive(Debug, Default)]
struct State {
    current_sample: usize,
    temp_end: usize,
    next_start: usize,
    prev_end: usize,
    triggered: bool,
    current_speech: FrameStamp,
    speeches: Vec<TimeStamp>,
    sample_rate: usize,
}

impl State {
    fn new(sample_rate: usize) -> Self {
        State {
            sample_rate,
            ..Default::default()
        }
    }

    fn update(&mut self, params: &Params, speech_prob: f32) {
        self.current_sample += params.frame_size_samples;

        if speech_prob > params.threshold {
            self.handle_speech_start(params, speech_prob);
            return;
        }

        if self.triggered && self.is_max_speech_duration_exceeded(params) {
            self.handle_max_speech_duration();
            return;
        }

        #[cfg(debug_assertions)]
        if self.is_speech_prob_near_threshold(speech_prob, params) {
            self.debug_speech_prob(speech_prob, params);
        }

        if self.triggered && speech_prob < (params.threshold - 0.15) {
            self.handle_speech_end(params, speech_prob);
        }
    }

    fn handle_speech_start(&mut self, params: &Params, speech_prob: f32) {
        if self.temp_end != 0 {
            self.temp_end = 0;
            if self.next_start < self.prev_end {
                self.next_start = self.current_sample.saturating_sub(params.frame_size_samples);
            }
        }
        if !self.triggered {
            #[cfg(debug_assertions)]
            self.debug(speech_prob, params, "start");
            self.triggered = true;
            self.current_speech.start = self.current_sample as i64 - params.frame_size_samples as i64;
        }
    }

    fn is_max_speech_duration_exceeded(&self, params: &Params) -> bool {
        (self.current_sample as i64 - self.current_speech.start) as f32 > params.max_speech_samples
    }

    fn handle_max_speech_duration(&mut self) {
        if self.prev_end > 0 {
            self.current_speech.end = self.prev_end as _;
            self.take_speech();
            if self.next_start < self.prev_end {
                self.triggered = false;
            } else {
                self.current_speech.start = self.next_start as _;
            }
        } else {
            self.current_speech.end = self.current_sample as _;
            self.take_speech();
            self.triggered = false;
        }
        self.reset_temporary_states();
    }

    fn is_speech_prob_near_threshold(&self, speech_prob: f32, params: &Params) -> bool {
        speech_prob >= (params.threshold - 0.15) && speech_prob < params.threshold
    }

    #[cfg(debug_assertions)]
    fn debug_speech_prob(&self, speech_prob: f32, params: &Params) {
        if self.triggered {
            self.debug(speech_prob, params, "speaking");
        } else {
            self.debug(speech_prob, params, "silence");
        }
    }

    fn handle_speech_end(&mut self, params: &Params, speech_prob: f32) {
        #[cfg(debug_assertions)]
        self.debug(speech_prob, params, "end");
        if self.temp_end == 0 {
            self.temp_end = self.current_sample;
        }
        if self.current_sample.saturating_sub(self.temp_end) > params.min_silence_samples_at_max_speech {
            self.prev_end = self.temp_end;
        }
        if self.current_sample.saturating_sub(self.temp_end) >= params.min_silence_samples {
            self.current_speech.end = self.temp_end as _;
            if self.current_speech.end - self.current_speech.start > params.min_speech_samples as _ {
                self.take_speech();
                self.reset_temporary_states();
                self.triggered = false;
            }
        }
    }

    fn reset_temporary_states(&mut self) {
        self.prev_end = 0;
        self.next_start = 0;
        self.temp_end = 0;
    }

    fn take_speech(&mut self) {
        let frame_stamp = std::mem::take(&mut self.current_speech);
        self.speeches.push(frame_stamp.to_timestamp(self.sample_rate));
    }

    fn check_for_last_speech(&mut self, last_sample: usize) {
        if self.current_speech.start > 0 {
            self.current_speech.end = last_sample as _;
            self.take_speech();
            self.prev_end = 0;
            self.next_start = 0;
            self.temp_end = 0;
            self.triggered = false;
        }
    }

    #[cfg(debug_assertions)]
    fn debug(&self, speech_prob: f32, params: &Params, title: &str) {
        if *DEBUG_SPEECH_PROB {
            let speech = self.current_sample as f32
                - params.frame_size_samples as f32
                - if title == "end" { params.speech_pad_samples } else { 0 } as f32;
            println!(
                "[{:10}: {:.3} s ({:.3}) {:8}]",
                title,
                speech / params.sample_rate as f32,
                speech_prob,
                self.current_sample - params.frame_size_samples,
            );
        }
    }
}
