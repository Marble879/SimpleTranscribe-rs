pub enum Model {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

impl Model {
    pub fn get_model(&self) -> &str {
        match self {
            Model::Tiny => "ggml-tiny",
            Model::Base => "ggml-base",
            Model::Small => "ggml-small",
            Model::Medium => "ggml-medium",
            Model::Large => "ggml-large",
        }
    }
}

impl std::str::FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tiny" => Ok(Model::Tiny),
            "base" => Ok(Model::Base),
            "small" => Ok(Model::Small),
            "medium" => Ok(Model::Medium),
            "large" => Ok(Model::Large),
            _ => Err(
                "Invalid model type. Supported types: tiny, base, small, medium, large".to_string(),
            ),
        }
    }
}
