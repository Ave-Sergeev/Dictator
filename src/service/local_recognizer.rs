use crate::pb::inference_pb::{Phrase, RecognizeResponse};
use crate::utils::word::{convert_word, UniversalWord};
use tonic::Status;
use vosk::{CompleteResult, Model, Recognizer};

pub struct LocalRecogniser {
    recognizer: Recognizer,
}

impl LocalRecogniser {
    pub fn new(model: &Model, sample_rate: f32, max_alternatives: u16) -> Result<Self, Status> {
        vosk::gpu_init();

        let mut recognizer = Recognizer::new(&model, sample_rate)
            .ok_or_else(|| Status::internal("Failed to create recognizer".to_string()))?;

        recognizer.set_words(true);
        recognizer.set_partial_words(true);
        recognizer.set_max_alternatives(max_alternatives);

        Ok(Self { recognizer })
    }

    pub fn transcribe(&mut self, audio: Vec<i16>) -> Result<RecognizeResponse, Status> {
        self.recognizer
            .accept_waveform(&audio)
            .map_err(|err| Status::internal(format!("Recognize error: {err}")))?;

        let result = self.recognizer.final_result();
        let recognize_response = Self::process(result);

        Ok(recognize_response)
    }

    fn process(complete_result: CompleteResult) -> RecognizeResponse {
        match complete_result {
            CompleteResult::Single(result) => Self::build_response(result.text, result.result),
            CompleteResult::Multiple(result) => result
                .alternatives
                .first()
                .map(|alternative| Self::build_response(alternative.text, alternative.result.clone()))
                .unwrap_or_default(),
        }
    }

    fn build_response(text: &str, words: impl IntoIterator<Item = impl UniversalWord>) -> RecognizeResponse {
        if text.is_empty() {
            return RecognizeResponse::default();
        }

        let pb_words = words.into_iter().map(|word| convert_word(&word)).collect();

        RecognizeResponse {
            phrases: vec![Phrase {
                text: text.to_string(),
                words: pb_words,
            }],
        }
    }
}
