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
