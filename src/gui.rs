use crate::{button::Button, GameObject, geometry::*, input::Mouse, TextureManager, FontManager, resource::Font};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

pub struct Gui  {
    add_circ : Button,
    font : Font,
}

impl Gui {
    pub fn new(btn_obj : GameObject, font : Font) -> Self {
        let add_circ = Button::new(btn_obj, Rect::new(0.0, 0.0, 100.0, 30.0), "add circuit".to_string());
        Gui {
            add_circ,
            font,
        }
    }

    fn draw_button<'sdl2, TTex, TFont>(
        canvas : &mut Canvas<Window>,
        texture_manager : &'sdl2 TextureManager<TTex>,
        font_manager : &'sdl2 FontManager<TFont>,
        font : &Font, button : &Button) -> Result<(), String> {
        texture_manager.draw(canvas, button.game_obj())?;
        font_manager.get_draw_at_vec2(
            font,
            button.text(),
            40,
            Vec2::new(
                button.game_obj().draw_rect.x,
                button.game_obj().draw_rect.y,
            ),
            Color::RGB(140, 80, 20)
        )?;
        Ok(())
    }

    pub fn draw<'sdl2, TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'sdl2 TextureManager<TTex>, font_manager : &'sdl2 FontManager<TFont>) -> Result<(), String> {
       Self::draw_button(canvas, texture_manager, font_manager, &self.font, &self.add_circ)?;
        
        Ok(())
    }

    pub fn update(&mut self, mouse : &Mouse) {
        self.add_circ.update(mouse);
    }

    pub fn add_circ_request(&self) -> Option<Rect> {
        if self.add_circ.clicked() {
            Some(Rect::new(100.0, 100.0, 50.0, 50.0))
        } else {
            None
        }
    }
}
