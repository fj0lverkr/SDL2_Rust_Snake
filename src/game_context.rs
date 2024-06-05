use crate::constants::{GRID_X_SIZE, GRID_Y_SIZE};
use rand::Rng;
use std::ops::Add;

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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn new() -> Point {
        let mut rng = rand::thread_rng();
        let rnd_x = rng.gen_range(0..GRID_X_SIZE);
        let rnd_y = rng.gen_range(0..GRID_Y_SIZE);
        Point(rnd_x as i32, rnd_y as i32)
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
}

impl Default for GameContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point::new(),
        }
    }

    pub fn do_next_tick(&mut self) {
        if let GameState::Over = self.state {
            self.player_position.pop();
        } else if let GameState::Playing = self.state {
            let current_player_head_pos = self.player_position.first().unwrap();
            let next_player_head_pos = match self.player_direction {
                PlayerDirection::Up => *current_player_head_pos + Point(0, -1),
                PlayerDirection::Down => *current_player_head_pos + Point(0, 1),
                PlayerDirection::Left => *current_player_head_pos + Point(-1, 0),
                PlayerDirection::Right => *current_player_head_pos + Point(1, 0),
            };

            if next_player_head_pos == self.food {
                self.player_position.push(Point(0, 0));
                self.food = Point::new();
            }

            for p in &self.player_position {
                if next_player_head_pos == *p {
                    self.state = GameState::Over;
                }
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
    }
}
