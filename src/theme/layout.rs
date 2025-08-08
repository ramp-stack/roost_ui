
#[derive(Clone, Debug)]
pub struct LayoutResources {
    pub content_max: f32,
    pub content_padding: f32,
    pub bumper_max: f32,
}

impl Default for LayoutResources {
    fn default() -> Self {
        LayoutResources {
            content_max: 375.0,
            content_padding: 24.0,
            bumper_max: 375.0,
        }
    }
}