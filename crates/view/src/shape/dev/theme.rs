use rand::Rng;

use bevy::prelude::*;

fn color_of_hex(hex: &str) -> Color {
    Srgba::hex(hex).unwrap().into()
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Resource)]
pub struct ViewShapeDevTheme {
    pub layout: LayoutDevTheme,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LayoutDevTheme {
    pub pivot_color: Color,
    pub anchor_color: Color,
    pub border_color: Color,
    pub pivot_radius: f32,
    pub anchor_radius: f32,
    pub border_line_width: f32,
}

impl Default for LayoutDevTheme {
    fn default() -> Self {
        Self {
            pivot_color: color_of_hex("FF0000"),
            anchor_color: color_of_hex("0000FF"),
            border_color: color_of_hex("00FF00"),
            pivot_radius: 16.0,
            anchor_radius: 16.0,
            border_line_width: 4.0,
        }
    }
}

impl LayoutDevTheme {
    pub fn get_view_color(&self) -> Color {
        let mut rng = rand::rng();
        let hue = rng.random_range(0.0..360.0);
        Hsla {
            hue,
            saturation: 0.5,
            lightness: 0.5,
            alpha: 0.5,
        }.into()
    }
}
