extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use sdl2_snake::constants::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};
use sdl2_snake::game_context::GameContext;
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.do_next_tick();
        }
        renderer.draw(&context)?;
    }

    Ok(())
}
