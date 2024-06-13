extern crate sdl2;
use sdl2::pixels::Color;

use crate::{
    constants::{GRID_X_SIZE, GRID_Y_SIZE},
    data_structs::Position2D,
    entities::text_elements::{FontDefinition, FontName, TextAlignment, TextElement, TextLine},
};
use rand::Rng;
use std::{fmt::Display, ops::Add};

pub enum GameState {
    Playing,
    Paused,
    Over,
}
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}
pub enum GameMode {
    Classic,
    WallPass,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            GameMode::Classic => write!(f, "Classic"),
            GameMode::WallPass => write!(f, "Wall Pass"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    fn new() -> Point {
        let mut rng = rand::thread_rng();
        let rnd_x = rng.gen_range(0..GRID_X_SIZE);
        let rnd_y = rng.gen_range(0..GRID_Y_SIZE);
        Point(rnd_x as i32, rnd_y as i32)
    }

    pub fn new_no_intersect(no_intersect: &Vec<Point>) -> Point {
        let mut new_point = Point::new();
        for p in no_intersect {
            if new_point == *p {
                new_point = Point::new_no_intersect(no_intersect);
            }
        }
        new_point
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
    pub mode: GameMode,
    pub score: i32,
    pub text_elements: Vec<TextElement>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut score_text_element = TextElement::new(
            String::from("score_element"),
            Position2D::new(5, 5),
            TextAlignment::Start,
            false,
        );
        let score_text_line = TextLine::new(
            String::from("score"),
            Position2D::new(5, 5),
            TextAlignment::Start,
            FontDefinition::new(FontName::ArcadeNormal, 24, Color::WHITE),
            String::from("0"),
        );
        score_text_element.lines.push(score_text_line);

        let mut mode_text_element = TextElement::new(
            String::from("mode_element"),
            Position2D::new(5, 5),
            TextAlignment::End,
            false,
        );
        let mode_text_line = TextLine::new(
            String::from("game_mode"),
            Position2D::new(5, 5),
            TextAlignment::Start,
            FontDefinition::new(FontName::ArcadeNormal, 24, Color::WHITE),
            String::from("Normal"),
        );
        mode_text_element.lines.push(mode_text_line);

        let mut pause_text_element = TextElement::new(
            String::from("pause_element"),
            Position2D::new(5, 5),
            TextAlignment::Start,
            true,
        );
        let pause_title_line = TextLine::new(
            String::from("title"),
            Position2D::new(0, 0),
            TextAlignment::Start,
            FontDefinition::new(FontName::ArcadeInterlaced, 48, Color::WHITE),
            String::from("Paused"),
        );
        let pause_esc_line = TextLine::new(
            String::from("esc_hint"),
            Position2D::new(0, 0),
            TextAlignment::Start,
            FontDefinition::new(FontName::ArcadeRounded, 24, Color::WHITE),
            String::from("Press ESC to unpause"),
        );
        let pause_mode_line = TextLine::new(
            String::from("mode_hint"),
            Position2D::new(0, 0),
            TextAlignment::Start,
            FontDefinition::new(FontName::ArcadeRounded, 24, Color::WHITE),
            String::from("Press M to change mode"),
        );
        pause_text_element.lines.push(pause_title_line);
        pause_text_element.lines.push(pause_esc_line);
        pause_text_element.lines.push(pause_mode_line);

        let half_x = (GRID_X_SIZE / 2) as i32;
        let half_y = (GRID_Y_SIZE / 2) as i32;

        GameContext {
            player_position: vec![
                Point(half_x, half_y),
                Point(half_x - 1, half_y),
                Point(half_x - 2, half_y),
            ],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            mode: GameMode::Classic,
            food: Point::new_no_intersect(&vec![
                Point(half_x, half_y),
                Point(half_x - 1, half_y),
                Point(half_x - 2, half_y),
            ]),
            score: 0,

            text_elements: vec![pause_text_element, score_text_element, mode_text_element],
        }
    }

    pub fn do_next_tick(&mut self) {
        if let GameState::Over = self.state {
            // Death animation
            self.player_position.pop();
        } else if let GameState::Playing = self.state {
            let mut player_is_alive = true;
            let current_player_head_pos = self.player_position.first().unwrap();
            let mut next_player_head_pos = match self.player_direction {
                PlayerDirection::Up => *current_player_head_pos + Point(0, -1),
                PlayerDirection::Down => *current_player_head_pos + Point(0, 1),
                PlayerDirection::Left => *current_player_head_pos + Point(-1, 0),
                PlayerDirection::Right => *current_player_head_pos + Point(1, 0),
            };

            // Detect snake collision with food
            if next_player_head_pos == self.food {
                self.player_position.push(Point(0, 0));
                self.score += 1;
                self.update_display_score();
                self.food = Point::new_no_intersect(&self.player_position);
            }

            // Detect snake collision with snake
            for p in &mut self.player_position {
                if next_player_head_pos == *p {
                    player_is_alive = false;
                    break;
                }
            }

            if !player_is_alive {
                self.game_over();
            }

            // Detect snake collision with walls
            let signed_grid_x_size = GRID_X_SIZE as i32;
            let signed_grid_y_size = GRID_Y_SIZE as i32;

            match self.mode {
                GameMode::Classic => {
                    if next_player_head_pos.0 == -1
                        || next_player_head_pos.0 == signed_grid_x_size + 1
                        || next_player_head_pos.1 == -1
                        || next_player_head_pos.1 == signed_grid_y_size + 1
                    {
                        self.game_over();
                    }
                }
                GameMode::WallPass => match self.player_direction {
                    PlayerDirection::Up => {
                        if next_player_head_pos.1 == -1 {
                            next_player_head_pos =
                                Point(next_player_head_pos.0, signed_grid_y_size);
                        }
                    }
                    PlayerDirection::Down => {
                        if next_player_head_pos.1 == signed_grid_y_size + 1 {
                            next_player_head_pos = Point(next_player_head_pos.0, 0);
                        }
                    }
                    PlayerDirection::Left => {
                        if next_player_head_pos.0 == -1 {
                            next_player_head_pos =
                                Point(signed_grid_x_size, next_player_head_pos.1);
                        }
                    }
                    PlayerDirection::Right => {
                        if next_player_head_pos.0 == signed_grid_x_size + 1 {
                            next_player_head_pos = Point(0, next_player_head_pos.1);
                        }
                    }
                },
            }

            self.player_position.pop();
            self.player_position.reverse();
            self.player_position.push(next_player_head_pos);
            self.player_position.reverse();
        }
    }

    pub fn move_player(&mut self, direction: PlayerDirection) {
        self.player_direction = direction;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Over => GameState::Over,
        };
        let pause_element_index = self
            .text_elements
            .iter()
            .position(|r| r.name == "pause_text_element")
            .unwrap_or(0);
        let pause_element = self.text_elements.get_mut(pause_element_index).unwrap();
        if let GameState::Paused = self.state {
            pause_element.visible = true;
        } else {
            pause_element.visible = false;
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            GameMode::WallPass => GameMode::Classic,
            GameMode::Classic => GameMode::WallPass,
        };

        self.score = 0;
        let new_player_position = vec![
            self.player_position[0],
            self.player_position[1],
            self.player_position[2],
        ];

        self.player_position = new_player_position;

        self.update_display_score();
        self.update_display_game_mode();
    }

    fn game_over(&mut self) {
        self.state = GameState::Over;
        println!("Final score: {}!", self.score);
    }

    fn update_display_score(&mut self) {
        let index = self
            .text_elements
            .iter()
            .position(|r| r.name == "score_element")
            .unwrap();
        let ui_score_text = self.text_elements.get_mut(index).unwrap();
        let index = ui_score_text
            .lines
            .iter()
            .position(|r| r.name == "score")
            .unwrap();
        let ui_score_text = ui_score_text.lines.get_mut(index).unwrap();
        ui_score_text.text = self.score.to_string();
    }

    fn update_display_game_mode(&mut self) {
        let index = self
            .text_elements
            .iter()
            .position(|r| r.name == "mode_element")
            .unwrap();
        let ui_mode_text = self.text_elements.get_mut(index).unwrap();
        let index = ui_mode_text
            .lines
            .iter()
            .position(|r| r.name == "game_mode")
            .unwrap();
        let ui_mode_text = ui_mode_text.lines.get_mut(index).unwrap();
        ui_mode_text.text = self.mode.to_string();
    }
}
