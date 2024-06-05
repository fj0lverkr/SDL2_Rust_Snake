extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2_snake::constants::{DOT_SIZE_IN_PXS, FRAMES_PER_SECOND, GRID_X_SIZE, GRID_Y_SIZE};
use sdl2_snake::game_context::{GameContext, GameState, PlayerDirection};
use sdl2_snake::renderer::Renderer;

fn main() -> Result<(), String> {
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

    let mut frame_counter = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::A | Keycode::Left => context.move_player(PlayerDirection::Left),
                    Keycode::W | Keycode::Up => context.move_player(PlayerDirection::Up),
                    Keycode::D | Keycode::Right => context.move_player(PlayerDirection::Right),
                    Keycode::S | Keycode::Down => context.move_player(PlayerDirection::Down),
                    Keycode::Escape => {
                        if let GameState::Over = context.state {
                            context = GameContext::new()
                        } else {
                            context.toggle_pause()
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND));

        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.do_next_tick();
        }
        renderer.draw(&context)?;
    }

    Ok(())
}
