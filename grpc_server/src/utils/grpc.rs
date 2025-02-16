use crate::pb::inference_pb::SpeechInterval;
use silero_vad::utils::utils::TimeStamp;

pub fn timestamp_to_speech_interval(timestamp: &TimeStamp) -> SpeechInterval {
    SpeechInterval {
        start_s: timestamp.start,
        end_s: timestamp.end,
    }
}
