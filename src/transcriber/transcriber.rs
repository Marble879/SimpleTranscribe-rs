use crate::audio_parser;
use crate::model;

#[derive(Debug)]
struct TranscriberOutput {
    start_timestamp: i64,
    end_timestamp: i64,
    text: String,
}

impl TranscriberOutput {
    fn get_start_timestamp(&self) -> &i64 {
        &self.start_timestamp
    }

    fn get_end_timestamp(&self) -> &i64 {
        &self.end_timestamp
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}

struct Transcriber {
    ctx: whisper_rs::WhisperContext,
}

impl Transcriber {
    pub fn new(model: model::model_handler::ModelHandler) -> Transcriber {
        Transcriber {
            ctx: whisper_rs::WhisperContext::new_with_params(
                &model.get_model_dir(),
                whisper_rs::WhisperContextParameters::default(),
            )
            .expect("failed to load model"),
        }
    }

    pub fn transcribe(
        &self,
        audio_path: &str,
        _output_path: &str,
        whisper_params: Option<whisper_rs::FullParams>,
    ) -> Result<TranscriberOutput, Box<dyn std::error::Error>> {
        let audio_data = audio_parser::audio_parser::parse_audio_file(audio_path);

        let mut state: whisper_rs::WhisperState =
            self.ctx.create_state().expect("Failed to create state");
        let params: whisper_rs::FullParams = match whisper_params {
            Some(whisper_params) => whisper_params,
            None => {
                whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 })
            }
        };

        state
            .full(params, &audio_data[..])
            .expect("failed to run the model");

        let mut transcribed_string = "".to_string();
        let mut start_timestamp = 0;
        let mut end_timestamp = 0;
        // fetch the results
        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .expect("failed to get segment");
            start_timestamp = state
                .full_get_segment_t0(i)
                .expect("failed to get segment start timestamp");
            end_timestamp = state
                .full_get_segment_t1(i)
                .expect("failed to get segment end timestamp");
            transcribed_string.push_str(&segment);
        }

        Ok(TranscriberOutput {
            start_timestamp,
            end_timestamp,
            text: transcribed_string,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::model_handler;

    use super::*;

    #[tokio::test]
    async fn component_test_happy_case() {
        let expected_result = " By what he is said and done, a man judges himself by what he is willing to do, by what he might have said, or might have done, a judgment that is necessarily hapered, but only by the scope and limits of his imagination, but by the ever-changing measure of his doubt and self-esteem.";

        let tiny_model_handler = model_handler::ModelHandler::new("Tiny", "models").await;
        let whisper_wrp = Transcriber::new(tiny_model_handler);

        let result = whisper_wrp
            .transcribe("src/test.mp3", "test.txt", None)
            .unwrap();
        let result_text = result.get_text();

        assert_eq!(expected_result, result_text);

        let _ = std::fs::remove_dir_all("models/");
    }
}
