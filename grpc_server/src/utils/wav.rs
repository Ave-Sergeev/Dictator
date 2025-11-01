use crate::error::error;
use hound::{SampleFormat, WavReader};

/// Reads samples from a WAV file and returns them as i16 values.
pub fn get_samples_from_wav(wav: &[u8]) -> error::Result<Vec<i16>> {
    let mut reader = WavReader::new(wav)?;

    let samples = reader.samples().flatten().collect();

    Ok(samples)
}

/// Converts a byte slice to a vector of i16 values (little-endian).
pub fn bytes_to_i16(bytes: &[u8]) -> Vec<i16> {
    if bytes.is_empty() {
        return Vec::new();
    }

    bytes
        .chunks_exact(2)
        .map(|chunk| {
            let bytes = <[u8; 2]>::try_from(chunk).unwrap();
            i16::from_ne_bytes(bytes)
        })
        .collect::<Vec<_>>()
}

/// Checks if WAV data is mono PCM format with the expected sample rate.
pub fn is_mono_pcm_wav(wav_data: &[u8], expected_sample_rate: u32) -> error::Result<bool> {
    let reader = WavReader::new(wav_data)?;
    let spec = reader.spec();

    let is_valid =
        spec.sample_format == SampleFormat::Int && spec.sample_rate == expected_sample_rate && spec.channels == 1;

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_samples_from_wav_empty_data() {
        let empty_wav = [];

        assert!(get_samples_from_wav(&empty_wav).is_err(), "Expected error for empty data, but got Ok");
    }

    #[test]
    fn test_get_samples_from_wav_invalid_data() {
        let invalid_wav = [0, 1, 2, 3];

        assert!(get_samples_from_wav(&invalid_wav).is_err(), "Expected error for invalid wav, but got Ok");
    }

    #[test]
    fn test_is_mono_pcm_wav_empty_data() {
        let empty_wav = [];

        assert!(is_mono_pcm_wav(&empty_wav, 16000).is_err(), "Expected error for empty data, but got Ok");
    }

    #[test]
    fn test_is_mono_pcm_wav_invalid_data() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];

        assert!(
            is_mono_pcm_wav(&invalid_data, 16000).is_err(),
            "Expected error for invalid data, but got Ok"
        );
    }

    #[test]
    fn test_bytes_to_i16_empty_input() {
        let bytes = [];
        let result = bytes_to_i16(&bytes);

        assert_eq!(result, Vec::<i16>::new());
    }

    #[test]
    fn test_bytes_to_i16_correct_conversion() {
        let bytes = [0x34, 0x12, 0x78, 0x56];
        let result = bytes_to_i16(&bytes);

        assert_eq!(result, vec![0x1234, 0x5678]);
    }

    #[test]
    fn test_bytes_to_i16_odd_length_truncated() {
        let bytes = [0x01, 0x02, 0x03];
        let result = bytes_to_i16(&bytes);

        assert_eq!(result, vec![0x0201]);
    }

    #[test]
    fn test_bytes_to_i16_endianness() {
        let bytes = if cfg!(target_endian = "little") {
            [0xCD, 0xAB]
        } else {
            [0xAB, 0xCD]
        };
        let expected = i16::from_ne_bytes(bytes);
        let result = bytes_to_i16(&bytes);

        assert_eq!(result, vec![expected]);
    }
}
