use zl001::{GameObject, resource};
use crate::input::Input;
use zl001::geometry::{Rect, Vec2};
pub struct Player {
    game_obj : GameObject,
}

impl Player {
    pub fn new(texture: resource::Texture) -> Self {
        Player {
            game_obj : GameObject {
                draw_rect: Rect::new(0.0, 0.0, texture.width as f64, texture.height as f64),
                tex_rect: None,
                tex : texture,
            }
        }
    }

    pub fn update(&mut self, elapsed: f64, input: &Input) {
        let mut vel = Vec2::new(0.0, 0.0);
        let speed = 500.0;
        if input.up {
            vel.y -= speed;
        }
        if input.down {
            vel.y += speed;
        }
        if input.left {
            vel.x -= speed;
        }
        if input.right {
            vel.x += speed;
        }
        self.game_obj.draw_rect.x += vel.x * elapsed;
        self.game_obj.draw_rect.y += vel.y * elapsed;
    }

    pub fn game_obj(&self) -> GameObject {
        self.game_obj.clone()
    }

}
