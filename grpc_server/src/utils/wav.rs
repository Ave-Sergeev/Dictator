use crate::error::error;

pub fn get_samples_from_wav(wav: &[u8]) -> error::Result<Vec<i16>> {
    let mut reader = hound::WavReader::new(wav)?;
    let samples: Vec<i16> = reader.samples().filter_map(|s| s.ok()).collect();

    Ok(samples)
}

pub fn bytes_to_i16(bytes: &[u8]) -> Vec<i16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| {
            let bytes = <[u8; 2]>::try_from(chunk).unwrap();
            i16::from_ne_bytes(bytes)
        })
        .collect()
}
