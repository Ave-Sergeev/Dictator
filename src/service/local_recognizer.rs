use crate::pb::inference_pb::{Phrase, RecognizeResponse, Word};
use serde_json::Value;
use tonic::Status;
use vosk::{Model, Recognizer};

pub struct LocalRecogniser {
    recognizer: Recognizer,
}

impl LocalRecogniser {
    pub fn new(model_path: &str, sample_rate: f32) -> Self {
        vosk::gpu_init();

        let model = Model::new(model_path).expect("Could not initialize Vosk model!");
        let mut recognizer = Recognizer::new(&model, sample_rate).expect("Failed to create recognizer");

        recognizer.set_words(true);
        recognizer.set_partial_words(true);

        Self { recognizer }
    }

    pub fn transcribe(&mut self, audio: Vec<i16>) -> Result<RecognizeResponse, Status> {
        for sample in audio.chunks(100) {
            self.recognizer
                .accept_waveform(sample)
                .map_err(|err| Status::internal(format!("Recognize error: {err}")))?;
            println!("{:#?}", self.recognizer.partial_result());
        }

        let result = self.recognizer.final_result();

        let result_str = serde_json::to_string(&result)
            .map_err(|err| Status::internal(format!("Failed to serialize CompleteResult: {err}")))?;
        let json_result: Value = serde_json::from_str(&result_str)
            .map_err(|err| Status::internal(format!("Failed to parse result JSON: {err}")))?;

        let mut recognize_response = RecognizeResponse::default();

        if let Some(text) = json_result["text"].as_str() {
            let mut phrase = Phrase {
                text: text.to_string(),
                words: vec![],
            };

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

            recognize_response.phrases.push(phrase);
        }

        Ok(recognize_response)
    }
}
