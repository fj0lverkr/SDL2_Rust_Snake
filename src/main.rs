extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::time::Duration;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

fn main() -> Result<(), String> {
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
    }

    pub struct Renderer {
        canvas: WindowCanvas,
    }

    impl Renderer {
        pub fn new(window: Window) -> Result<Renderer, String> {
            let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
            Ok(Renderer { canvas })
        }

        pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
            self.draw_bg(context);
            self.draw_player(context)?;
            self.canvas.present();

            Ok(())
        }

        fn draw_bg(&mut self, context: &GameContext) {
            let color = match context.state {
                GameState::Playing => Color::RGB(0, 0, 0),
                GameState::Paused => Color::RGB(30, 30, 30),
                GameState::Over => Color::RGB(60, 60, 60),
            };

            self.canvas.set_draw_color(color);
            self.canvas.clear();
        }

        fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
            self.canvas.set_draw_color(Color::GREEN);

            for point in &context.player_position {
                self.draw_point(point)?;
            }
            Ok(())
        }

        fn draw_point(&mut self, point: &Point) -> Result<(), String> {
            let Point(x, y) = point;
            self.canvas.fill_rect(Rect::new(
                x * DOT_SIZE_IN_PXS as i32,
                y * DOT_SIZE_IN_PXS as i32,
                DOT_SIZE_IN_PXS,
                DOT_SIZE_IN_PXS,
            ))?;

            Ok(())
        }
    }

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let window = video_subsys
        .window(
            "Snake Game SDL2 Rust",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();

    let mut renderer = Renderer::new(window)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.draw(&context)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
