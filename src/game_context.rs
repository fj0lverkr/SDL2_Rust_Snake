extern crate sdl2;

use crate::{
    constants::{GRID_X_SIZE, GRID_Y_SIZE},
    data::text_data,
    entities::text_elements::TextElement,
};
use rand::Rng;
use std::{collections::HashMap, fmt::Display, ops::Add};

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
    pub text_elements: HashMap<String, TextElement>,
}

impl Default for GameContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GameContext {
    pub fn new() -> GameContext {
        let text_elements = text_data::get_game_context_text();
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
            text_elements,
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
        let pause_element = self.text_elements.get_mut("pause_element").unwrap();
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
        let game_over_text_element = self.text_elements.get_mut("game_over_element").unwrap();
        game_over_text_element.visible = true;
        println!("Final score: {}!", self.score);
    }

    fn update_display_score(&mut self) {
        let ui_score_text = self.text_elements.get_mut("score_element").unwrap();
        let ui_score_text = ui_score_text.lines.get_mut("score_line").unwrap();
        ui_score_text.text = self.score.to_string();
    }

    fn update_display_game_mode(&mut self) {
        let ui_mode_text = self.text_elements.get_mut("mode_element").unwrap();
        let ui_mode_text = ui_mode_text.lines.get_mut("game_mode_line").unwrap();
        ui_mode_text.text = self.mode.to_string();
    }
}
