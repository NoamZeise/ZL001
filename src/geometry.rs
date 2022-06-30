use sdl2::rect;

#[derive(Clone)]
pub struct Rect {
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rect { x, y, w, h }
    }
    pub fn new_from_sdl_rect(sdl_rect : &rect::Rect) -> Self {
        Rect {
            x: sdl_rect.x as f64,
            y: sdl_rect.y as f64,
            w: sdl_rect.w as f64,
            h: sdl_rect.h as f64
        }
    }
    pub fn to_sdl_rect(&self) -> rect::Rect {
        rect::Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

#[derive(Clone)]
pub struct Vec2 {
    pub x : f64,
    pub y : f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}
