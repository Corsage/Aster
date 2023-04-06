use bevy::prelude::Color;

pub struct Colors;
impl Colors {
    pub fn print_color(color: Color) {
        print!(
            "\x1b[38;2;{};{};{}m",
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8
        );
        print!(
            "\n{};{};{}\n",
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8
        );
        print!(
            "RGBA: {}, {}, {}, {}",
            color.r(),
            color.g(),
            color.b(),
            color.a()
        );
        println!("\x1b[0m");
    }
}
