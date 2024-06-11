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
    pub is_overlay: bool,
}

impl TextElement {
    pub fn new(
        name: String,
        pos_x: i32,
        pos_y: i32,
        font_name: FontName,
        font_size: u16,
        color: Color,
        text: String,
        is_overlay: bool,
    ) -> TextElement {
        TextElement {
            name,
            pos_x,
            pos_y,
            font_name,
            font_size,
            color,
            text,
            is_overlay,
        }
    }
}
