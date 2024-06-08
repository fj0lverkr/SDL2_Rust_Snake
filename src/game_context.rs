use crate::constants::{GRID_X_SIZE, GRID_Y_SIZE};
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
    pub mode: GameMode,
    pub score: i32,
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
            mode: GameMode::Classic,
            food: Point::new(),
            score: 0,
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
                self.score += 1;
                self.player_position.push(Point(0, 0));
                self.food = Point::new();
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
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            GameMode::WallPass => GameMode::Classic,
            GameMode::Classic => GameMode::WallPass,
        };

        println!("Gamemode changed to {}", self.mode);
    }

    fn game_over(&mut self) {
        self.state = GameState::Over;
        println!("Final score: {}!", self.score);
    }
}
