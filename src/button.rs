use crate::input::Mouse;
use crate::geometry::*;
use crate::GameObject;

pub struct Button {
    clicked : bool,
    selected : bool,
    prev_mouse : Mouse,
    game_obj : GameObject,
    text : String,
}

impl Button {
    pub fn new(game_obj : GameObject, rect : Rect, text : String) -> Self {
        let mut game_obj = game_obj;
        game_obj.draw_rect = rect;
        Button {
            clicked : false,
            selected : false,
            prev_mouse : Mouse::new(),
            game_obj,
            text
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    /// set clicked to true for a single frame if button is clicked
    pub fn update(&mut self, mouse : &Mouse) {
        self.selected = self.game_obj.draw_rect.contains(&Vec2::new(mouse.x as f64, mouse.y as f64));
        self.clicked =  self.selected && mouse.left_click && !self.prev_mouse.left_click;
        self.prev_mouse = *mouse;
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn game_obj(&self) -> &GameObject {
        &self.game_obj
    }
}
