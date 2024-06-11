extern crate sdl2;
use sdl2::pixels::Color;

use crate::data_structs::Position2D;

pub enum FontName {
    ArcadeInterlaced,
    ArcadeNormal,
    ArcadeRounded,
}

pub struct FontDefinition {
    pub font_name: FontName,
    pub font_size: u16,
    pub color: Color,
}

pub struct TextLine {
    pub name: String,
    pub position: Position2D,
    pub font: FontDefinition,
    pub text: String,
}

pub struct TextElement {
    pub name: String,
    pub position: Position2D,
    pub lines: Vec<TextLine>,
    pub is_overlay: bool,
}

impl FontDefinition {
    pub fn new(font_name: FontName, font_size: u16, color: Color) -> FontDefinition {
        FontDefinition {
            font_name,
            font_size,
            color,
        }
    }
}

impl TextLine {
    pub fn new(name: String, position: Position2D, font: FontDefinition, text: String) -> TextLine {
        TextLine {
            name,
            position,
            font,
            text,
        }
    }
}

impl TextElement {
    pub fn new(name: String, position: Position2D, is_overlay: bool) -> TextElement {
        TextElement {
            name,
            position,
            lines: vec![],
            is_overlay,
        }
    }
}
