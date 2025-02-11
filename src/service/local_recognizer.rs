use crate::pb::inference_pb::{Phrase, RecognizeResponse, Word};
use tonic::Status;
use vosk::{Model, Recognizer};

pub struct LocalRecogniser {
    recognizer: Recognizer,
}

impl LocalRecogniser {
    pub fn new(model: &Model, sample_rate: f32) -> Self {
        vosk::gpu_init();

        let mut recognizer = Recognizer::new(&model, sample_rate).expect("Failed to create recognizer");

        recognizer.set_words(true);
        recognizer.set_partial_words(true);

        Self { recognizer }
    }

    pub fn transcribe(&mut self, audio: Vec<i16>) -> Result<RecognizeResponse, Status> {
        self.recognizer
            .accept_waveform(&audio)
            .map_err(|err| Status::internal(format!("Recognize error: {err}")))?;

        let result = self
            .recognizer
            .final_result()
            .single()
            .ok_or_else(|| Status::internal("Failed to get recognition result"))?;

        let mut recognize_response = RecognizeResponse::default();

        if !result.text.is_empty() {
            let mut phrase = Phrase {
                text: result.text.to_string(),
                words: Vec::with_capacity(result.result.len()),
            };

            phrase.words = result
                .result
                .into_iter()
                .map(|word_info| Word {
                    word: word_info.word.to_string(),
                    start_ms: (word_info.start * 1000.0) as i64,
                    end_ms: (word_info.end * 1000.0) as i64,
                })
                .collect();

            recognize_response.phrases.push(phrase);
        }

        Ok(recognize_response)
    }
}
