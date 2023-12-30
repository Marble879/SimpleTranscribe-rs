fn transcribe(&self, model: model::model::Model) {
    // TODO: In for loop at end, append to string and then return a final string
    let model_path = format!("{}/{}.bin", self.models_dir, self.model.get_model());
    let context = whisper_rs::WhisperContext::new_with_params(
        &model_path,
        whisper_rs::WhisperContextParameters::default(),
    )
    .expect("failed to load model");

    let params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
    let audio_data = vec![0_f32; 16000 * 2];

    let mut state = context.create_state().expect("Failed to create state");
    state
        .full(params, &audio_data[..])
        .expect("failed to run model");

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

#[cfg(test)]
mod tests {
    use crate::model::model_handler;

    use super::*;

    #[tokio::test]
    async fn component_test_happy_case() {
        // TODO: Make assert the return string after fulfill to do in try_use_model
        let tiny_model =
            model_handler::ModelHandler::new(model::model::Model::Tiny, "models".to_string());
        tiny_model.setup_model().await;

        tiny_model.transcribe();
    }
}
