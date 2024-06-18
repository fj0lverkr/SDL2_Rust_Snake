// Define text elements and lines here
extern crate sdl2;
use sdl2::pixels::Color;

use std::collections::HashMap;

use crate::{
    data_structs::Position2D,
    entities::text_elements::{FontDefinition, FontName, TextAlignment, TextElement, TextLine},
};

pub fn get_game_context_text() -> HashMap<String, TextElement> {
    let mut text_elements: HashMap<String, TextElement> = HashMap::new();

    let mut score_text_element =
        TextElement::new(0, Position2D::new(5, 5), TextAlignment::Start, false);
    let score_text_line = TextLine::new(
        0,
        Position2D::new(5, 5),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeNormal, 24, Color::WHITE),
        String::from("0"),
    );
    score_text_element
        .lines
        .insert("score_line".to_string(), score_text_line);

    let mut mode_text_element =
        TextElement::new(0, Position2D::new(5, 5), TextAlignment::End, false);
    let mode_text_line = TextLine::new(
        0,
        Position2D::new(5, 5),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeNormal, 24, Color::WHITE),
        String::from("Normal"),
    );
    mode_text_element
        .lines
        .insert("game_mode_line".to_string(), mode_text_line);

    // PAUSE MENU TEXT
    let mut pause_text_element =
        TextElement::new(1, Position2D::new(5, 5), TextAlignment::Start, true);
    let pause_title_line = TextLine::new(
        0,
        Position2D::new(0, 0),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeInterlaced, 48, Color::WHITE),
        String::from("Paused"),
    );
    let pause_esc_line = TextLine::new(
        1,
        Position2D::new(0, 0),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeRounded, 24, Color::WHITE),
        String::from("Press ESC to continue"),
    );
    let pause_mode_line = TextLine::new(
        2,
        Position2D::new(0, 0),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeRounded, 24, Color::WHITE),
        String::from("Press M to change mode"),
    );

    pause_text_element
        .lines
        .insert("pause_title_line".to_string(), pause_title_line);
    pause_text_element
        .lines
        .insert("pause_esc_line".to_string(), pause_esc_line);
    pause_text_element
        .lines
        .insert("pause_mode_line".to_string(), pause_mode_line);

    // GAME OVER MENU TEXT
    let mut game_over_text_element =
        TextElement::new(1, Position2D::new(5, 5), TextAlignment::Start, true);
    game_over_text_element.visible = false;
    let game_over_title_line = TextLine::new(
        0,
        Position2D::new(0, 0),
        TextAlignment::Start,
        FontDefinition::new(FontName::ArcadeInterlaced, 48, Color::WHITE),
        "Game Over".to_string(),
    );

    game_over_text_element
        .lines
        .insert("game_over_title_line".to_string(), game_over_title_line);

    text_elements.insert("pause_element".to_string(), pause_text_element);
    text_elements.insert("game_over_element".to_string(), game_over_text_element);
    text_elements.insert("score_element".to_string(), score_text_element);
    text_elements.insert("mode_element".to_string(), mode_text_element);
    text_elements
}
