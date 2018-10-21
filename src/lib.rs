#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate graphics;
extern crate opengl_graphics;
extern crate vecmath;

mod animation;

use std::path::Path;
use std::collections::HashMap;
use graphics::{ ImageSize };
use graphics::math::{ Matrix2d };
use graphics::types::SourceRectangle;
use opengl_graphics::{ GlGraphics, Texture, TextureSettings };
use vecmath::{ Vector3 };

pub use animation::Animation;

#[derive(Debug)]
pub enum Orientation {
    Normal,
    Flipped,
}

pub struct SpriteSheet<'a> {
    pub sprite_texture: Texture,
    pub width: u32,
    pub height: u32,
    pub animations: HashMap<String, Animation>,
    scale: (f64, f64),
    pos: Vector3<f64>,
    rect: [f64; 4],
    src_rect: SourceRectangle,
    playing: Option<&'a str>,
    current_frame: usize,
    orient_horz: Orientation,
    orient_vert: Orientation,
}

impl<'a> SpriteSheet<'a> {
    pub fn new<P>(filepath: P) -> SpriteSheet<'a>
        where P: AsRef<Path>
    {
        let texture = Texture::from_path(filepath, &TextureSettings::new()).unwrap();

        let (w, h) = texture.get_size();
        let src_rect: SourceRectangle = [0.0, 0.0, w as f64, h as f64];

        SpriteSheet {
            sprite_texture: texture,
            width: w,
            height: h,
            pos: [0.0, 0.0, 0.0],
            scale: (1.0, 1.0),
            rect: src_rect,
            src_rect,
            animations: HashMap::new(),
            playing: None,
            current_frame: 0,
            orient_horz: Orientation::Normal,
            orient_vert: Orientation::Normal,
        }
    }

    pub fn set_src_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        let src_rect: SourceRectangle = [x, y, w, h];
        self.src_rect = src_rect;
    }

    pub fn set_scale(&mut self, sx: f64, sy: f64) {
        self.scale = (sx, sy);
    }

    pub fn get_scale(self) -> (f64, f64) {
        let (sx, sy) = self.scale;

        (sx, sy)
    }

    pub fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.pos = [x, y, z];
    }

    pub fn get_pos(self) -> Vector3<f64> {
        let [px, py, pz] = self.pos;

        [px, py, pz]
    }

    pub fn set_frame_size(&mut self, w: f64, h: f64) {
        let src_rect: SourceRectangle = [self.src_rect[0], self.src_rect[1], w, h];

        self.rect = [0.0, 0.0, w, h];
        self.src_rect = src_rect;
    }

    pub fn set_frame_view(&mut self, x: f64, y: f64) {
        let src_rect: SourceRectangle = [x, y, self.src_rect[2], self.src_rect[3]];

        self.src_rect = src_rect;
    }

    pub fn set_orientation(&mut self, horz: Orientation, vert: Orientation) {
        self.orient_horz = horz;
        self.orient_vert = vert;
    }

    pub fn set_orientation_h(&mut self, horz: Orientation) {
        self.orient_horz = horz;
    }

    pub fn set_orientation_v(&mut self, vert: Orientation) {
        self.orient_vert = vert;
    }

    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.name.to_string(), animation);
    }

    pub fn add_animations(&mut self, animations: &mut HashMap<String, Animation>) {
       for (name, anim) in animations.drain() {
           self.animations.insert(name, anim);
       }
    }

    pub fn remove_animation(&mut self, name: &str) {
        self.animations.remove(name);
    }

    pub fn clear_animations(&mut self) {
        self.animations.clear();
    }

    pub fn play(&mut self, name: &'a str) {
        self.playing = Some(name); // .unwrap();
    }

    pub fn cancel(&mut self, frame: Option<usize>) {
        self.playing = None;
        self.current_frame = frame.unwrap_or(0);
    }

    fn update_animation_frame(&mut self, name: &str) -> (f64, f64) {
        let Animation { frames, .. } = self.animations.get(name).unwrap();
        let frame_out_of_bounds = self.current_frame >= (frames.len() - 1);
        let frame = self.current_frame.clone();

        self.current_frame = if frame_out_of_bounds {
            0
        } else {
            self.current_frame + 1
        };

        let (x, y) = *frames.get(frame).unwrap_or(&(0.0, 0.0));

        (x, y)
    }

    pub fn render(&mut self, t: Matrix2d, g: &mut GlGraphics, &ext_dt: &f64) {
        use graphics::*;

        let (sx, sy) = self.scale;
        let [px, py, _] = self.pos;
        let [_, _, w, h] = self.src_rect;
        let is_playing = self.playing.clone();
        let (fx, fy) = match is_playing {
            Some(ref a) => self.update_animation_frame(a),
            None => (0.0, 0.0),
        };

        self.set_frame_view(fx, fy);

        let image = Image::new()
            .rect(self.rect)
            .src_rect(self.src_rect);

        let ref draw_state: graphics::DrawState = Default::default();

        // + w * 2, + h * 2 as offset for flipping
        let width_offset = w * 2.0;
        let height_offset = h * 2.0;
        let transform = match self { // maybe should scale -> rotate -> translate
            SpriteSheet {
                orient_horz: Orientation::Normal,
                orient_vert: Orientation::Normal,
                ..
            } => t.trans(px, py).scale(sx, sy),
            SpriteSheet {
                orient_horz: Orientation::Flipped,
                orient_vert: Orientation::Normal,
                ..
            } => t.trans(px + width_offset, py).scale(sx, sy).flip_h(),
            SpriteSheet {
                orient_horz: Orientation::Normal,
                orient_vert: Orientation::Flipped,
                ..
            } => t.trans(px, py + height_offset).scale(sx, sy).flip_v(),
            SpriteSheet {
                orient_horz: Orientation::Flipped,
                orient_vert: Orientation::Flipped,
                ..
            } => t.trans(px + width_offset, py + height_offset).scale(sx, sy).flip_hv(),
            _ => t.trans(px, py).scale(sx, sy)
        };

        image.draw(&self.sprite_texture, draw_state, transform, g);
    }
}
