use crate::model;

struct Whisper {
    ctx: whisper_rs::WhisperContext,
}

impl Whisper {
    pub fn new(model: model::model_handler::ModelHandler) -> Whisper {
        Whisper {
            ctx: whisper_rs::WhisperContext::new_with_params(
                &model.get_model_dir(),
                whisper_rs::WhisperContextParameters::default(),
            )
            .expect("failed to load model"),
        }
    }

    pub fn transcribe(
        &self,
        _audio_path: &str,
        _output_path: &str,
        whisper_params: Option<whisper_rs::FullParams>,
    ) {
        let mut state: whisper_rs::WhisperState =
            self.ctx.create_state().expect("Failed to create state");
        let audio_data = vec![0_f32; 16000 * 2];

        let params: whisper_rs::FullParams;
        match whisper_params {
            Some(whisper_params) => params = whisper_params,
            None => {
                params =
                    whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 })
            }
        }

        state
            .full(params, &audio_data[..])
            .expect("failed to run the model");

        // fetch the results
        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");
        for i in 0..num_segments {
            let segment = state
                .full_get_segment_text(i)
                .expect("failed to get segment");
            let start_timestamp = state
                .full_get_segment_t0(i)
                .expect("failed to get segment start timestamp");
            let end_timestamp = state
                .full_get_segment_t1(i)
                .expect("failed to get segment end timestamp");
            println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
        }
    }

    fn create_state() {}
}

#[cfg(test)]
mod tests {
    use crate::model::model_handler;

    use super::*;

    #[tokio::test]
    async fn component_test_happy_case() {
        // TODO: Make assert the return string after fulfill to do in try_use_model
        let tiny_model_handler = model_handler::ModelHandler::new("Tiny", "models").await;
        //tiny_model_handler.await;

        let whisper_wrp = Whisper::new(tiny_model_handler);

        whisper_wrp.transcribe("test.wav", "test.txt", None);
    }
}
