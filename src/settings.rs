pub struct Vec2f {
    pub x: f64,
    pub y: f64
}

pub struct Settings {
    pub wind_size: Vec2f,
    pub cell_size: Vec2f,
    pub font_size: u32,
    pub text_offset: Vec2f
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            wind_size: Vec2f{ x: 900.0, y: 900.0 },
            cell_size: Vec2f{ x: 100.0, y: 100.0 },
            font_size: 64,
            text_offset: Vec2f{ x: 30.0, y: 75.0 }
        }
    }
}
