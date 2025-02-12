use crate::pb::inference_pb::{Phrase, RecognizeResponse, Word};
use crate::utils::word::{convert_word, UniversalWord};
use tonic::Status;
use vosk::{CompleteResult, Model, Recognizer};

pub struct LocalRecogniser {
    recognizer: Recognizer,
    pause_threshold: i64,
    split_into_phrases: bool,
}

impl LocalRecogniser {
    pub fn new(
        model: &Model,
        sample_rate: f32,
        max_alternatives: u16,
        pause_threshold: &i64,
        split_into_phrases: bool,
    ) -> Result<Self, Status> {
        vosk::gpu_init();

        let mut recognizer = Recognizer::new(&model, sample_rate)
            .ok_or_else(|| Status::internal("Failed to create recognizer".to_string()))?;

        recognizer.set_words(true);
        recognizer.set_partial_words(true);
        recognizer.set_max_alternatives(max_alternatives);

        Ok(Self {
            recognizer,
            pause_threshold: *pause_threshold,
            split_into_phrases,
        })
    }

    pub fn transcribe(&mut self, audio: Vec<i16>) -> Result<RecognizeResponse, Status> {
        self.recognizer
            .accept_waveform(&audio)
            .map_err(|err| Status::internal(format!("Recognize error: {err}")))?;

        let result = self.recognizer.final_result();
        let recognize_response = Self::process(result, self.pause_threshold, self.split_into_phrases);

        Ok(recognize_response)
    }

    fn process(complete_result: CompleteResult, pause_threshold: i64, split_into_phrases: bool) -> RecognizeResponse {
        match complete_result {
            CompleteResult::Single(result) => {
                Self::create_response(result.text, result.result, pause_threshold, split_into_phrases)
            }
            CompleteResult::Multiple(result) => {
                if let Some(alternative) = result.alternatives.first() {
                    Self::create_response(
                        alternative.text,
                        alternative.result.clone(),
                        pause_threshold,
                        split_into_phrases,
                    )
                } else {
                    RecognizeResponse::default()
                }
            }
        }
    }

    fn create_response(
        text: &str,
        words: impl IntoIterator<Item = impl UniversalWord>,
        pause_threshold: i64,
        split_into_phrases: bool,
    ) -> RecognizeResponse {
        if text.is_empty() {
            return RecognizeResponse::default();
        }

        let pb_words = words.into_iter().map(|word| convert_word(&word)).collect();
        let phrases = match split_into_phrases {
            true => Self::group_phrases(pb_words, pause_threshold),
            false => vec![Phrase {
                text: text.to_string(),
                words: pb_words,
            }],
        };

        RecognizeResponse {
            phrases,
            text: text.to_string(),
        }
    }

    fn group_phrases(words: Vec<Word>, pause_threshold: i64) -> Vec<Phrase> {
        let mut phrases = Vec::new();
        let mut current_phrase = Phrase::default();

        for word in words {
            if let Some(prev_end) = current_phrase.words.last() {
                let pause = word.start_ms - prev_end.end_ms;
                if pause > pause_threshold {
                    current_phrase.text = current_phrase.text.trim().to_string();
                    phrases.push(std::mem::take(&mut current_phrase));
                }
            }

            current_phrase.words.push(word.clone());
            current_phrase.text.push_str(&word.word);
            if !current_phrase.text.ends_with(' ') {
                current_phrase.text.push(' ');
            }
        }

        if !current_phrase.words.is_empty() {
            current_phrase.text = current_phrase.text.trim().to_string();
            phrases.push(current_phrase);
        }

        phrases
    }
}
