use crate::pb::inference_pb::Word;
use vosk::{Word as VoskWord, WordInAlternative};

pub fn convert_word(word: &impl UniversalWord) -> Word {
    Word {
        word: word.word().to_string(),
        start_ms: (word.start() * 1000.0) as i64,
        end_ms: (word.end() * 1000.0) as i64,
    }
}

pub trait UniversalWord {
    fn word(&self) -> &str;
    fn start(&self) -> f32;
    fn end(&self) -> f32;
}

impl UniversalWord for VoskWord<'_> {
    fn word(&self) -> &str {
        &self.word
    }
    fn start(&self) -> f32 {
        self.start
    }
    fn end(&self) -> f32 {
        self.end
    }
}

impl UniversalWord for WordInAlternative<'_> {
    fn word(&self) -> &str {
        &self.word
    }
    fn start(&self) -> f32 {
        self.start
    }
    fn end(&self) -> f32 {
        self.end
    }
}
