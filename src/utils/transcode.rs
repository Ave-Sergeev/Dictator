pub fn pcm_s16be_to_pcm_s16le(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    for i in 0..input.len() {
        if i % 2 == 0 {
            output.push(input[i + 1]);
            output.push(input[i]);
        }
    }
    output
}
