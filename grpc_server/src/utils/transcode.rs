/// Converts PCM audio data from 16-bit big-endian (S16BE) format to 16-bit little-endian (S16LE) format.
pub fn pcm_s16be_to_pcm_s16le(input: &[u8]) -> Vec<u8> {
    if !input.len().is_multiple_of(2) {
        log::warn!("Warning: Input length is odd. The last byte will be ignored");
    }

    input
        .chunks_exact(2)
        .flat_map(|chunk| [chunk[1], chunk[0]])
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let input: &[u8] = &[];
        let output = pcm_s16be_to_pcm_s16le(&input);

        assert_eq!(output, Vec::<u8>::new());
    }

    #[test]
    fn test_even_length_input() {
        let input = [0x12, 0x34, 0x56, 0x78];
        let output = pcm_s16be_to_pcm_s16le(&input);

        assert_eq!(output, vec![0x34, 0x12, 0x78, 0x56]);
    }

    #[test]
    fn test_odd_length_input() {
        let input = [0x12, 0x34, 0x56];
        let output = pcm_s16be_to_pcm_s16le(&input);

        assert_eq!(output, vec![0x34, 0x12]);
    }

    #[test]
    fn test_single_sample() {
        let input = [0x12, 0x34];
        let output = pcm_s16be_to_pcm_s16le(&input);

        assert_eq!(output, vec![0x34, 0x12]);
    }

    #[test]
    fn test_multiple_samples() {
        let input = [0x00, 0x01, 0xFF, 0xFE, 0x7F, 0xFF];
        let output = pcm_s16be_to_pcm_s16le(&input);

        assert_eq!(output, vec![0x01, 0x00, 0xFE, 0xFF, 0xFF, 0x7F]);
    }
}
