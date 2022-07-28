use crate::input::Mouse;
use crate::geometry::*;
use crate::GameObject;

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
}
