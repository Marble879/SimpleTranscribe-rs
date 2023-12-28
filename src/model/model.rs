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
            Model::Tiny => "tiny",
            Model::Base => "base",
            Model::Small => "small",
            Model::Medium => "medium",
            Model::Large => "large",
        }
    }
}
