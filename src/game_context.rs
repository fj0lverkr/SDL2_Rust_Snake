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

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

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
            food: Point(3, 3),
        }
    }

    pub fn do_next_tick(&mut self) {
        let current_player_head_pos = self.player_position.first().unwrap();
        let next_player_head_pos = match self.player_direction {
            PlayerDirection::Up => *current_player_head_pos + Point(0, -1),
            PlayerDirection::Down => *current_player_head_pos + Point(0, 1),
            PlayerDirection::Left => *current_player_head_pos + Point(-1, 0),
            PlayerDirection::Right => *current_player_head_pos + Point(1, 0),
        };

        self.player_position.pop();
        self.player_position.reverse();
        self.player_position.push(next_player_head_pos);
        self.player_position.reverse();
    }
}
