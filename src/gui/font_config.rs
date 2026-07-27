
pub struct FontConfig {
    heading_size:f32,
    font_size:f32,
    button_font_size:f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            heading_size:24.0,
            font_size:14.0,
            button_font_size:16.0,
        }
    }
}

impl FontConfig {
    pub fn heading_size(&self) -> f32 {
        self.heading_size
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn button_font_size(&self) -> f32 {
        self.button_font_size
    }
}

