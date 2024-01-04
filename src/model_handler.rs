pub struct ModelHandler {
    model_name: String, // list of downloaded models
    models_dir: String, // path to the models directory
}

const MODEL_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "tiny" => "ggml-tiny",
    "base" => "ggml-base",
    "small" => "ggml-small",
    "medium" => "ggml-medium",
    "large" => "ggml-large",
};

impl ModelHandler {
    pub async fn new(model_name: &str, models_dir: &str) -> ModelHandler {
        let model_handler = ModelHandler {
            model_name: MODEL_MAP
                .get(&model_name.to_lowercase())
                .copied()
                .unwrap()
                .to_string(),
            models_dir: models_dir.to_string(),
        };

        if model_handler.is_model_existing() {
            return model_handler;
        }

        let _ = model_handler.setup_directory();
        let _ = model_handler.download_model().await;

        model_handler
    }

    /// setup the directory to which models will be downloaded.
    /// Sets a global vx
    ///
    /// # Returns
    ///
    /// * `Void` - directory is setup.
    fn setup_directory(&self) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&self.models_dir);
        if !path.exists() {
            let _ = std::fs::create_dir_all(path)?;
        }
        Ok(())
    }

    fn is_model_existing(&self) -> bool {
        match std::fs::metadata(format!("{}/{}.bin", self.models_dir, self.model_name)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Download the specified model.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model to download.
    ///
    /// # Returns
    ///
    /// * `Void` - The model is downloaded to the models directory.
    async fn download_model(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_model_existing() {
            self.setup_directory()?;
        }
        let base_url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";
        let response = reqwest::get(format!("{}/{}.bin", base_url, &self.model_name)).await?;
        let mut file =
            std::fs::File::create(format!("{}/{}.bin", &self.models_dir, &self.model_name))?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }

    pub fn get_model_dir(&self) -> String {
        format!("{}/{}.bin", &self.models_dir, &self.model_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::model_handler;

    use super::*;

    #[tokio::test]
    async fn test_check_model_exists_existent_path() {
        let path = std::path::Path::new("test_models/ggml-tiny.bin");
        if !path.exists() {
            let _ = std::fs::create_dir_all(path);
        }

        let test_model = model_handler::ModelHandler::new("tiny", "test_models/").await;
        let result = test_model.is_model_existing();
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_setup_directory_happy_case() {
        let path = std::path::Path::new("test_models/ggml-tiny.bin");
        if !path.exists() {
            let _ = std::fs::create_dir_all(path);
        }

        let test_model = model_handler::ModelHandler::new("tiny", "test_models/").await;
        let result = test_model.setup_directory();
        assert_eq!(result.is_ok(), true);
        let _ = std::fs::remove_dir_all("test_models/");
    }

    #[tokio::test]
    async fn test_download_model_happy_case() {
        fn prep_test_dir() {
            let path = std::path::Path::new("test_dir/");
            if !path.exists() {
                let _ = std::fs::create_dir_all(path);
            }
        }

        prep_test_dir();

        let model_handler = model_handler::ModelHandler::new("tiny", "test_dir/").await;

        let _result = model_handler.download_model().await;

        let is_file_existing = match std::fs::metadata("test_dir/ggml-tiny.bin") {
            Ok(_) => true,
            Err(_) => false,
        };

        assert_eq!(is_file_existing, true);

        let _ = std::fs::remove_dir_all("test_dir/");
    }
}
