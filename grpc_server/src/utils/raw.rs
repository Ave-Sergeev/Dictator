pub fn is_pcm_raw(raw_data: &[u8]) -> bool {
    if raw_data.is_empty() || raw_data.len() % 2 != 0 {
        false
    } else {
        true
    }
}
