use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::video::Window;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::ttf;

use std::collections::HashMap;
use std::path::Path;
use std::clone::Clone;
pub mod geometry;
use crate::geometry::{Rect, Vec2};
pub mod input;
pub mod player;


pub mod resource {
//! represent sdl2 textures and fonts as cheap structs that hold indexes for resource managers

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
    loaded_texture_paths : HashMap<String,  usize>,
    textures     : Vec<Texture<'a>>,
}

impl<'a, T> TextureManager<'a, T> {
    pub fn new(tex_creator: &'a TextureCreator<T>) -> Self {

        TextureManager {
            texture_creator : tex_creator,
            loaded_texture_paths: HashMap::new(),
            textures : Vec::new(),
        }
    }

    pub fn load(&mut self, path : &Path) -> Result<resource::Texture, String> {
        let path_as_string = path.to_string_lossy().to_string();
        let tex_index = match self.loaded_texture_paths.contains_key(&path_as_string) {
            true => self.loaded_texture_paths[&path_as_string],
            false => {
                self.textures.push(self.texture_creator.load_texture(path)?);
                self.loaded_texture_paths.insert(path_as_string, self.textures.len() - 1);
                self.textures.len() - 1
            },
        };
        let last_tex = &self.textures[tex_index];
        Ok(
        resource::Texture {
            id: tex_index,
            width: last_tex.query().width,
            height: last_tex.query().height,
        })

    }

    pub fn draw(&self, canvas : &mut Canvas<Window>, game_obj: &GameObject) -> Result<(), String> {
        canvas.copy(
            &self.textures[game_obj.tex.id],
            match &game_obj.tex_rect {
                Some(r) => Some(r.to_sdl_rect()),
                None => None
            },
            game_obj.draw_rect.to_sdl_rect()
        )
    }
}

/// can be returned by `FontManager`, stores an sdl2 texture and a rect for drawing to a canvas
pub struct TextDraw<'a> {
    pub tex  : sdl2::render::Texture<'a>,
    pub rect : sdl2::rect::Rect,
}

const FONT_LOAD_SIZE : u16 = 128;

/// Stores 'sdl2::ttf::Font' and returns textures or draws them
pub struct FontManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    ttf_context: &'a ttf::Sdl2TtfContext,
    loaded_font_paths : HashMap<String, usize>,
    pub fonts : Vec<ttf::Font<'a, 'static>>,
}

impl<'a, T> FontManager<'a, T> {
    pub fn new(ttf_context : &'a ttf::Sdl2TtfContext, texture_creator : &'a TextureCreator<T>) -> Result<Self, String> {
        Ok(FontManager {
            texture_creator,
            ttf_context,
            loaded_font_paths: HashMap::new(),
            fonts : Vec::new(),
        })
    }

    pub fn load_font(&mut self, path : &Path) -> Result<resource::Font, String>{
        let path_string = path.to_string_lossy().to_string();
        let font_index = match self.loaded_font_paths.contains_key(&path_string) {
            true => self.loaded_font_paths[&path_string],
            false => {
                self.fonts.push(
                    match self.ttf_context.load_font(path, FONT_LOAD_SIZE) {
                        Ok(s) => s,
                        Err(e) => { return Err(e.to_string()); }
                    }
                );
                self.loaded_font_paths.insert(path_string, self.fonts.len() - 1);
                self.fonts.len() - 1
            }
        };
        Ok(
            resource::Font {
            id: font_index,
        })
    }
    /// return a `TextDraw` that has a corrected `rect.width` based on the supplied height and the rendered font
    pub fn get_draw(&self, font: &resource::Font, text: &str, height : u32) -> Result<TextDraw, String> {
       let surface = match self.fonts[font.id]
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255)) {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
        };
        let tex = match self.texture_creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(e) => { return Err(e.to_string()); },
        };
        let ratio = tex.query().height as f64 / tex.query().width as f64;
        Ok(
        TextDraw {
            tex,
            rect: sdl2::rect::Rect::new(0, 0, (height as f64 / ratio) as u32, height),
        })
    }

    /// draws the supplied text to the canvas in the supplied font at the given height and position
    pub fn draw(&self, canvas : &mut Canvas<Window>, font : &resource::Font, text: &str, height : u32, pos : Vec2) -> Result<(), String> {
        if text.len() == 0 { return Ok(()); }
        let mut tex_draw = self.get_draw(font, text, height)?;
        tex_draw.rect.x = pos.x as i32;
        tex_draw.rect.y = pos.y as i32;
        canvas.copy(&tex_draw.tex, None, tex_draw.rect)
    }
}
