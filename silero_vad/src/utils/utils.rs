use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum SampleRate {
    EightKHz,
    SixteenKHz,
}

impl From<SampleRate> for i64 {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::EightKHz => 8000,
            SampleRate::SixteenKHz => 16000,
        }
    }
}

impl From<SampleRate> for usize {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::EightKHz => 8000,
            SampleRate::SixteenKHz => 16000,
        }
    }
}

impl From<usize> for SampleRate {
    fn from(value: usize) -> Self {
        match value {
            8000 => SampleRate::EightKHz,
            16000 => SampleRate::SixteenKHz,
            _ => panic!("Unsupported sample rate: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VadParams {
    pub frame_size: usize,
    pub threshold: f32,
    pub min_silence_duration_ms: usize,
    pub speech_pad_ms: usize,
    pub min_speech_duration_ms: usize,
    pub max_speech_duration_s: f32,
    pub sample_rate: usize,
}

impl Default for VadParams {
    fn default() -> Self {
        Self {
            frame_size: 64,
            threshold: 0.5,
            min_silence_duration_ms: 0,
            speech_pad_ms: 64,
            min_speech_duration_ms: 64,
            max_speech_duration_s: f32::INFINITY,
            sample_rate: 16000,
        }
    }
}

#[derive(Debug, Default)]
pub struct FrameStamp {
    pub start: i64,
    pub end: i64,
}

impl FrameStamp {
    pub fn to_timestamp(&self, sample_rate: usize) -> TimeStamp {
        TimeStamp {
            start: self.start as f64 / sample_rate as f64,
            end: self.end as f64 / sample_rate as f64,
        }
    }
}

impl Display for FrameStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[start:{}, end:{}]", self.start, self.end)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TimeStamp {
    pub start: f64,
    pub end: f64,
}
