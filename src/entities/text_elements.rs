extern crate sdl2;
use std::collections::HashMap;

use sdl2::pixels::Color;

use crate::data_structs::Position2D;

pub enum FontName {
    ArcadeInterlaced,
    ArcadeNormal,
    ArcadeRounded,
}

pub enum TextAlignment {
    Start,
    Center,
    End,
}

pub struct FontDefinition {
    pub font_name: FontName,
    pub font_size: u16,
    pub color: Color,
}

pub struct TextLine {
    pub vertical_order: u32,
    pub position: Position2D,
    pub alignment: TextAlignment,
    pub font: FontDefinition,
    pub text: String,
}

pub struct TextElement {
    pub name: String,
    pub visible: bool,
    pub position: Position2D,
    pub alignment: TextAlignment,
    pub lines: HashMap<String, TextLine>,
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
    pub fn new(
        vertical_order: u32,
        position: Position2D,
        alignment: TextAlignment,
        font: FontDefinition,
        text: String,
    ) -> TextLine {
        TextLine {
            vertical_order,
            position,
            alignment,
            font,
            text,
        }
    }
}

impl TextElement {
    pub fn new(
        name: String,
        position: Position2D,
        alignment: TextAlignment,
        is_overlay: bool,
    ) -> TextElement {
        TextElement {
            name,
            visible: true,
            position,
            alignment,
            lines: HashMap::new(),
            is_overlay,
        }
    }
}
