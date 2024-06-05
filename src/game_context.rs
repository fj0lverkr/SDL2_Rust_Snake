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

pub struct Point(pub i32, pub i32);

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
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

    pub fn do_next_tick(&mut self) {}
}
