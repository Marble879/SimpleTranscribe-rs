use crate::model;

struct ModelHandler {
    model: model::model::Model, // list of downloaded models
    models_dir: String,         // path to the models directory
}

impl ModelHandler {
    fn new(models_dir: String, model: model::model::Model) -> ModelHandler {
        ModelHandler { model, models_dir }
    }

    pub fn setup_model(&self) {
        if Self::check_model_exists(&self.models_dir) {
            return;
        }; // verify if the model already exists before downloading
        Self::setup_directory(&self.models_dir);
        let model_name = self.model.get_model();
        Self::download_model(model_name, &self.models_dir);
    }

    /// setup the directory to which models will be downloaded.
    /// Sets a global vx
    ///
    /// # Returns
    ///
    /// * `Void` - directory is setup.
    fn setup_directory(dir: &str) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(dir);
        if !path.exists() {
            std::fs::create_dir_all(path);
        }
        Ok(())
    }

    fn check_model_exists(models_path: &str) -> bool {
        match std::fs::metadata(models_path) {
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
    async fn download_model(
        model_name: &str,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get("").await?;
        let mut file = std::fs::File::create(format!("{path}/{model_name}.bin"))?;
        let mut content = std::io::Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::model_handler;

    use super::*;

    #[test]
    fn test_check_model_exists_non_existent_path() {
        let result = model_handler::ModelHandler::check_model_exists("nonExistentPath");
        println!("{}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_model_exists_existent_path() {
        let result = model_handler::ModelHandler::check_model_exists("src");
        println!("{}", result);
        assert_eq!(result, true);
    }
}
