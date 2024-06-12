extern crate sdl2;

use crate::constants::{DOT_SIZE_IN_PXS, FONT_PATH, GRID_X_SIZE, GRID_Y_SIZE};
use crate::entities::text_elements::FontName;
use crate::game_context::{GameContext, GameState, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};

pub struct Renderer {
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_blend_mode(BlendMode::Blend);
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
        for element in &context.text_elements {
            if element.is_overlay {
                self.create_overlay();
            }
            for (i, text) in element.lines.iter().enumerate() {
                let font_name = match text.font.font_name {
                    FontName::ArcadeInterlaced => "ArcadeInterlaced-O4d.ttf",
                    FontName::ArcadeNormal => "ArcadeNormal-ZDZ.ttf",
                    FontName::ArcadeRounded => "ArcadeRounded-3DM.ttf",
                };
                let font_path = format!("{FONT_PATH}{font_name}");
                let font = self.ttf_context.load_font(font_path, text.font.font_size)?;

                // render a surface, and convert it to a texture bound to the canvas
                let surface = font
                    .render(text.text.as_str())
                    .blended(text.font.color)
                    .map_err(|e| e.to_string())?;
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let TextureQuery { width, height, .. } = texture.query();

                let mut target_x = element.position.x + text.position.x;
                let mut target_y = element.position.y + text.position.y;

                if element.is_overlay {
                    let mut line_factor = (element.lines.len() / 2) as i32;
                    if line_factor == 0 {
                        line_factor = 1;
                    }
                    target_x = (((GRID_X_SIZE * DOT_SIZE_IN_PXS) / 2) - (width / 2)) as i32;
                    target_y = ((GRID_Y_SIZE * DOT_SIZE_IN_PXS) / 2) as i32
                        - (element.lines.len() as i32 / line_factor - i as i32) * height as i32;
                }

                let target = Rect::new(target_x, target_y, width, height);

                self.canvas.copy(&texture, None, Some(target))?;
            }
        }
        Ok(())
    }
    fn create_overlay(&mut self) {
        let width = GRID_X_SIZE * DOT_SIZE_IN_PXS;
        let height = GRID_Y_SIZE * DOT_SIZE_IN_PXS;
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 200));
        let overlay = Rect::new(0, 0, width, height);
        self.canvas.fill_rect(overlay).unwrap();
    }
}
