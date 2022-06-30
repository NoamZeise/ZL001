use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::video::Window;
use sdl2::image::LoadTexture;

use std::collections::HashMap;
use std::path::Path;
use std::clone::Clone;
pub mod geometry;
use crate::geometry::Rect;

pub mod resource {
    #[derive(Clone)]
    pub struct Texture {
        pub id:     usize,
        pub width:  u32,
        pub height: u32
    }
}

#[derive(Clone)]
pub struct GameObject {
    pub draw_rect : Rect,
    pub tex_rect : Option<Rect>,
    pub tex  : resource::Texture,
}

impl GameObject {
    pub fn new(texture: resource::Texture) -> Self {
        GameObject {
            draw_rect: Rect::new(0.0, 0.0, texture.width as f64, texture.height as f64),
            tex_rect : None,
            tex: texture,
        }
    }
}

pub struct TextureManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    loaded_textures : HashMap<String, Texture<'a>>,
    texture_ids     : Vec<String>,
}

impl<'a, T> TextureManager<'a, T> {
    pub fn new(tex_creator: &'a TextureCreator<T>) -> Self {

        TextureManager {
            texture_creator : tex_creator,
            loaded_textures: HashMap::new(),
            texture_ids : Vec::new(),
        }
    }

    pub fn load(&mut self, path : &Path) -> Result<resource::Texture, String> {
        let path_string = path.to_string_lossy().to_string();
        self.loaded_textures.insert(path_string.clone(), self.texture_creator.load_texture(path)?);
        self.texture_ids.push(path_string);
        let last_tex_index = self.texture_ids.len() - 1;
        let last_tex = &self.loaded_textures[&self.texture_ids[last_tex_index]];
        Ok(
        resource::Texture {
            id: last_tex_index,
            width: last_tex.query().width,
            height: last_tex.query().height,
        }
        )
    }

    pub fn draw(&self, canvas : &mut Canvas<Window>, game_obj: &GameObject) -> Result<(), String> {
        canvas.copy(
            &self.loaded_textures[&self.texture_ids[game_obj.tex.id]],
            match &game_obj.tex_rect {
                Some(r) => Some(r.to_sdl_rect()),
                None => None
            },
            game_obj.draw_rect.to_sdl_rect()
        )
    }
}
