use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::video::Window;
use sdl2::surface::Surface;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::ttf;

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
    #[derive(Clone)]
    pub struct Font {
        pub id : usize,
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
        })
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

const FONT_LOAD_SIZE : u16 = 128;

pub struct FontManager<'a> {
    ttf_context: ttf::Sdl2TtfContext,
    loaded_fonts : HashMap<String, ttf::Font<'a, 'a>>,
    font_ids : Vec<String>,
}

impl<'a> FontManager<'a> {
    pub fn new() -> Result<Self, String> {
        let ttf_context = match ttf::init() {
          Ok(t) => t,
          Err(e) => { return Err(e.to_string()); }
        };

        Ok(FontManager {
            ttf_context,
            loaded_fonts: HashMap::new(),
            font_ids : Vec::new(),
        })
    }

    pub fn load_font(&'a mut self, path : &Path) -> Result<resource::Font, String>{
        let path_string = path.to_string_lossy().to_string();
        self.loaded_fonts.insert(
            path_string.clone(),
            match self.ttf_context.load_font(path, FONT_LOAD_SIZE) {
                Ok(s) => s,
                Err(e) => { return Err(e.to_string()); }
            }
        );
        self.font_ids.push(path_string);
        let last_font_index = self.font_ids.len() - 1;
        Ok(
            resource::Font {
            id: last_font_index,
        })
    }

    pub fn get_surface(&self, font: resource::Font, text: &str) -> Result<Surface, String> {
        match self.loaded_fonts[&self.font_ids[font.id]]
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255)) {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string()),
        }
    }
}
