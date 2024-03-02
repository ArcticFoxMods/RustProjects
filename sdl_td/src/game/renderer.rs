use sdl2::pixels::Color;

use super::{point::Point, constants, context::GameContext, state::GameState};


pub struct Renderer { canvas: sdl2::render::WindowCanvas }

impl Renderer {
    pub fn new(window: sdl2::video::Window) -> Result <Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer{canvas})
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(sdl2::rect::Rect::new(
            x * constants::DOT_SIZE_IN_PXS as i32,
            y * constants::DOT_SIZE_IN_PXS as i32,
            constants::DOT_SIZE_IN_PXS,
            constants::DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();

        Ok(())

    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Playing => Color::RGB(0,0,0),
            GameState::Paused => Color::RGB(30,30,30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);

        for point in &context.player_position {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;

        Ok(())
    }
}