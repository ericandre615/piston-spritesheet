extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::path::Path;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

extern crate spritesheet;
use spritesheet::{ Animation, Orientation };

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub struct App<'a> {
    gl: GlGraphics,
    megaman_sheet: spritesheet::SpriteSheet<'a>,
    mega_x: f64,
    mega_y: f64,
    mega_vel: f64,
    anim: f64,
    dirty: bool,
}

impl<'a> App<'a> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        let megaman = &mut self.megaman_sheet;

        self.gl.draw(args.viewport(), | c, g | {
            clear(GREEN, g);
            megaman.render(c.transform, g, &args.ext_dt);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let megaman = &mut self.megaman_sheet;
        let mega_x = self.mega_x.clone();
        let mega_y = self.mega_y.clone();
        let mega_vel = self.mega_vel.clone();

        megaman.set_frame_size(70.0, 70.0);
        megaman.set_frame_view(0.0, 0.0);
        megaman.set_scale(2.0, 2.0);

        if (mega_x >= 1024.0) {
            self.mega_x = -70.0;
        }

        if (mega_y >= 600.0) {
            if(!self.dirty) {
                megaman.play("idle");
                self.dirty = true;
            }
        } else {
            megaman.play("jump");
            self.mega_y = mega_y + 4.0;
            self.mega_x = mega_x;
        }

        self.mega_x += self.mega_vel;

        megaman.set_pos(self.mega_x, self.mega_y, 0.0);
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Left => {
                    self.mega_vel = -3.0;
                    self.megaman_sheet.play("walk");
                    self.megaman_sheet.set_orientation_h(Orientation::Flipped);
                }
                Key::Right => {
                    self.mega_vel = 3.0;
                    self.megaman_sheet.play("walk");
                    self.megaman_sheet.set_orientation_h(Orientation::Normal);
                }
                Key::LCtrl => {
                    self.megaman_sheet.set_orientation(Orientation::Flipped, Orientation::Flipped);
                }
                Key::RCtrl => {
                    self.megaman_sheet.set_orientation(Orientation::Normal, Orientation::Normal);
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                _ => {
                    self.mega_vel = 0.0;
                    self.megaman_sheet.cancel(None);
                    self.megaman_sheet.play("idle");
                }
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "demo  spritesheet",
        [SCREEN_WIDTH, SCREEN_HEIGHT]
    )
    .opengl(opengl)
    .vsync(true)
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut megaman_sheet = spritesheet::SpriteSheet::new(Path::new("assets/sprites/megaman_spritesheet_two.png"));
    let megaman_anim_data = Animation::load_from_json("assets/data/megaman.animations.json");

    megaman_sheet.add_animations(&mut megaman_anim_data.unwrap());
    megaman_sheet.play("jump");

    let mut app = App {
        gl: GlGraphics::new(opengl),
        megaman_sheet: megaman_sheet,
        mega_x: 400.0,
        mega_y: -70.0,
        mega_vel: 0.0,
        anim: 10.0,
        dirty: false,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            app.press(&p);
        }

        if let Some(r) = e.release_args() {
            app.release(&r);
        }
    }
}
