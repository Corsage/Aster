use bevy::prelude::*;

pub const TITLE_SIZE: f32 = 64.0;

pub const FONT_MONO_BOLD: &str = "fonts\\FiraCode-Bold.ttf";

pub fn get_title_style(server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: server.load(FONT_MONO_BOLD),
        font_size: TITLE_SIZE,
        color: Color::WHITE,
    }
}
