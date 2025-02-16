pub fn pcm_s16be_to_pcm_s16le(input: &[u8]) -> Vec<u8> {
    if input.len() % 2 != 0 {
        eprintln!("Warning: Input length is odd. The last byte will be ignored");
    }

    input
        .chunks_exact(2)
        .flat_map(|chunk| [chunk[1], chunk[0]].into_iter())
        .collect()
}
