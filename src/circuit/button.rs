use crate::{
    GameObject, geometry::*, input::Mouse, TextureManager, FontManager, resource::Font};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

pub struct Button {
    clicked : bool,
    selected : bool,
    game_obj : GameObject,
    text : String,
}

impl Button {
    pub fn new(game_obj : GameObject, rect : Option<Rect>, text : String) -> Self {
        let mut game_obj = game_obj;
        if let Some(r) = rect {
            game_obj.draw_rect = r;
        }
        Button {
            clicked : false,
            selected : false,
            game_obj,
            text
        }
    }

    pub fn reset(&mut self) {
        self.clicked = false;
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// set clicked to true for a single frame if button is clicked
    pub fn update(&mut self, mouse : &Mouse, prev_mouse : &Mouse) {
        self.selected = self.game_obj.draw_rect.contains(&Vec2::new(mouse.x as f64, mouse.y as f64));
        self.clicked =  self.selected && mouse.left_click && !prev_mouse.left_click;
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn game_obj(&self) -> &GameObject {
        &self.game_obj
    }

    pub fn has_text(&self) -> bool {
        self.text != ""
    }

    pub fn draw<'sdl2, TTex, TFont>(&self,
        canvas : &mut Canvas<Window>,
        texture_manager : &'sdl2 TextureManager<TTex>,
        font_manager : &'sdl2 FontManager<TFont>,
        font : &Font) -> Result<(), String> {
        texture_manager.draw(canvas, self.game_obj())?;
        if self.has_text() {
            let draw = font_manager.get_draw_at_vec2(
                font,
                self.text(),
                (self.game_obj().draw_rect.h * 0.9) as u32,
                Vec2::new(
                    self.game_obj().draw_rect.x + self.game_obj().draw_rect.w * 0.03,
                    self.game_obj().draw_rect.y,
                ),
                Color::RGB(140, 80, 20)
            )?;
            canvas.copy(&draw.tex, None, draw.rect)?;
        }
        if self.selected() {
            texture_manager.draw_rect(canvas, &self.game_obj().draw_rect, &Rect::new(40.0, 40.0, 40.0, 80.0))?;
        }
        Ok(())
    }
}
