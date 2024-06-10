extern crate sdl2;

use crate::constants::{DOT_SIZE_IN_PXS, FONT_PATH, GRID_X_SIZE, GRID_Y_SIZE};
use crate::entities::text_element::FontName;
use crate::game_context::{GameContext, GameState, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};

pub struct Renderer {
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        Ok(Renderer {
            canvas,
            ttf_context,
        })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();

        self.draw_bg(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.draw_text_elements(context, &texture_creator)?;
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

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_point(&context.food)?;
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
    fn draw_text_elements(
        &mut self,
        context: &GameContext,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        for text in &context.text_elements {
            let font_name = match text.font_name {
                FontName::ArcadeInterlaced => "ArcadeInterlaced-O4d.ttf",
                FontName::ArcadeNormal => "ArcadeNormal-ZDZ.ttf",
                FontName::ArcadeRounded => "ArcadeRounded-3DM.ttf",
            };
            let font_path = format!("{FONT_PATH}{font_name}");
            let font = self.ttf_context.load_font(font_path, text.font_size)?;

            // render a surface, and convert it to a texture bound to the canvas
            let surface = font
                .render(text.text.as_str())
                .blended(text.color)
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let TextureQuery { width, height, .. } = texture.query();

            // If the example text is too big for the screen, downscale it (and center irregardless)
            let padding = 1;
            let target =
                self.get_centered_rect(width, height, GRID_X_SIZE - padding, GRID_Y_SIZE - padding);

            self.canvas.copy(&texture, None, Some(target))?;
        }
        Ok(())
    }

    // Scale fonts to a reasonable size when they're too big (though they might look less smooth)
    fn get_centered_rect(
        &self,
        rect_width: u32,
        rect_height: u32,
        cons_width: u32,
        cons_height: u32,
    ) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                let h = (rect_height as f32 / wr) as u32;
                (cons_width, h)
            } else {
                let w = (rect_width as f32 / hr) as u32;
                (w, cons_height)
            }
        } else {
            (rect_width, rect_height)
        };

        let cx = (GRID_X_SIZE as i32 - w as i32) / 2;
        let cy = (GRID_Y_SIZE as i32 - h as i32) / 2;
        Rect::new(cx, cy, w, h)
    }
}
