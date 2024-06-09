extern crate sdl2;
use sdl2::pixels::Color;

pub enum FontName {
    ArcadeInterlaced,
    ArcadeNormal,
    ArcadeRounded,
}

pub struct TextElement {
    pub name: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub font_name: FontName,
    pub font_size: u16,
    pub color: Color,
    pub text: String,
}
