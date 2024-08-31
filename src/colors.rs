use iced::Color;

pub const RED: Color = Color {
    r: 1.0,
    g: 0.1960784314,
    b: 0.1960784314,
    a: 1.0,
};

pub const BLUE: Color = Color {
    r: 0.1960784314,
    g: 0.1960784314,
    b: 1.0,
    a: 1.0,
};

pub const YELLOW: Color = Color {
    r: 0.9882352941,
    g: 1.0,
    b: 0.6196078431,
    a: 1.0,
};

pub const LIGHT_GRAY: Color = Color {
    r: 0.75,
    g: 0.75,
    b: 0.75,
    a: 1.0,
};

pub fn blend_colors(first: Color, second: Color, a: f32) -> Color {
    let x = 1.0 - a;
    Color::from_rgb(
        first.r * x + second.r * a,
        first.g * x + second.g * a,
        first.b * x + second.b * a,
    )
}