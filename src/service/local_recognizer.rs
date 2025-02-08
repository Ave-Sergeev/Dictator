use crate::pb::inference_pb::{Phrase, Phrases, Word};
use serde_json::Value;
use vosk::{Model, Recognizer};

pub struct LocalRecogniser {
    recognizer: Recognizer,
}

impl LocalRecogniser {
    pub fn new(model_path: &str, sample_rate: f32) -> Self {
        let model = Model::new(model_path).unwrap();
        let mut recognizer = Recognizer::new(&model, sample_rate).unwrap();

        recognizer.set_words(true);
        recognizer.set_partial_words(true);

        Self { recognizer }
    }

    pub fn transcribe(&mut self, audio: Vec<i16>) -> Phrases {
        for sample in audio.chunks(100) {
            self.recognizer.accept_waveform(sample).expect("Processing error");
            println!("{:#?}", self.recognizer.partial_result());
        }

        let result = self.recognizer.final_result();

        let mut phrases = Phrases::default();

        let result_str = serde_json::to_string(&result).expect("Failed to serialize CompleteResult");
        let json_result: Value = serde_json::from_str(&result_str).expect("Failed to parse result JSON");

        if let Some(text) = json_result["text"].as_str() {
            let mut phrase = Phrase::default();
            phrase.text = text.to_string();

            if let Some(words) = json_result["result"].as_array() {
                for word_obj in words {
                    if let (Some(word), Some(start), Some(end)) =
                        (word_obj["word"].as_str(), word_obj["start"].as_f64(), word_obj["end"].as_f64())
                    {
                        let word = Word {
                            word: word.to_string(),
                            start_ms: (start * 1000.0) as i64,
                            end_ms: (end * 1000.0) as i64,
                        };
                        phrase.words.push(word);
                    }
                }
            }

            phrases.phrases.push(phrase);
        }

        phrases
    }
}
