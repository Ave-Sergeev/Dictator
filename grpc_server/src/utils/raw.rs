/// Checks if the raw data is PCM raw audio data.
/// PCM raw data must be non-empty and of even length (16-bit samples).
pub fn is_pcm_raw(raw_data: &[u8]) -> bool {
    !raw_data.is_empty() && raw_data.len().is_multiple_of(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_data() {
        assert!(!is_pcm_raw(&[]), "Empty data should not be valid PCM");
    }

    #[test]
    fn test_odd_length() {
        assert!(!is_pcm_raw(&[0_u8, 1, 2]), "Odd-length data should be invalid PCM");
    }

    #[test]
    fn test_valid_pcm() {
        assert!(is_pcm_raw(&[0_u8, 0]), "Should be valid PCM");
        assert!(is_pcm_raw(&[0xFF_u8, 0x7F]), "Should be valid PCM");
        assert!(is_pcm_raw(&[0_u8, 0, 0, 0]), "Should be valid PCM");
    }

    #[test]
    fn test_large_valid_pcm() {
        let large_data = vec![0_u8; 4096];

        assert!(is_pcm_raw(&large_data), "Large even-sized buffer should be valid");

        let mut non_zero_data = vec![0_u8; 4096];

        non_zero_data[0] = 1;
        non_zero_data[1] = 1;

        assert!(is_pcm_raw(&non_zero_data), "Large non-zero buffer should be valid");
    }

    #[test]
    fn test_all_zeros() {
        assert!(is_pcm_raw(&[0_u8, 0, 0, 0]), "All-zero data should be technically valid PCM");
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_pcm_raw(&[0_u8]), "Single byte should be invalid");
        assert!(is_pcm_raw(&[0_u8, 0, 0x80, 0x7F]), "Boundary value samples should be valid");
    }
}
