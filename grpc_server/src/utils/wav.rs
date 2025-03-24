use crate::error::error;
use hound::{SampleFormat, WavReader};

pub fn get_samples_from_wav(wav: &[u8]) -> error::Result<Vec<i16>> {
    let mut reader = WavReader::new(wav)?;

    let samples = reader.samples().filter_map(|sample| sample.ok()).collect::<Vec<_>>();

    Ok(samples)
}

pub fn bytes_to_i16(bytes: &[u8]) -> Vec<i16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| {
            let bytes = <[u8; 2]>::try_from(chunk).unwrap();
            i16::from_ne_bytes(bytes)
        })
        .collect::<Vec<_>>()
}

pub fn is_mono_pcm_wav(wav_data: &[u8], expected_sample_rate: u32) -> error::Result<bool> {
    let reader = WavReader::new(wav_data)?;
    let spec = reader.spec();

    let is_valid =
        spec.sample_format == SampleFormat::Int && spec.sample_rate == expected_sample_rate && spec.channels == 1;

    Ok(is_valid)
}
