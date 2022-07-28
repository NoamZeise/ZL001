use crate::{
    GameObject, geometry::*, input::Mouse, TextureManager, FontManager, resource::Font};
use super::button::Button;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub struct CodeGui  {
    circuit_btn : Button,   
    prev_mouse : Mouse,
    current_mouse_pos : Vec2,
    font : Font,
}

impl CodeGui {
    pub fn new(btn_obj : GameObject, font : Font) -> Self {
        let circuit_btn = Button::new(btn_obj.clone(), Some(Rect::new(5.0, 440.0, 110.0, 30.0)), "circuit".to_string());
        CodeGui {
            circuit_btn,
            prev_mouse : Mouse::new(),
            current_mouse_pos : Vec2::new(0.0, 0.0),
            font,
        }
    }
    
    pub fn draw<'sdl2, TTex, TFont>(&mut self, canvas : &mut Canvas<Window>,  texture_manager : &'sdl2 TextureManager<TTex>, font_manager : &'sdl2 FontManager<TFont>) -> Result<(), String> {
        self.circuit_btn.draw(canvas, texture_manager, font_manager, &self.font)?;
        Ok(())
    }

    pub fn update(&mut self, mouse : &Mouse) {
        self.current_mouse_pos = Vec2::new(mouse.x as f64, mouse.y as f64);

        self.btn_update(mouse);
            
        
        self.prev_mouse = *mouse;
    }
    
    fn btn_update(&mut self, mouse : &Mouse) {

        self.circuit_btn.update(mouse, &self.prev_mouse);
    }

    pub fn circuit_btn(&mut self) -> bool {
        let r = self.circuit_btn.clicked();
        self.circuit_btn.reset();
        r
    }
}
