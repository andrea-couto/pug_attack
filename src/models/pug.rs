extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use piston::input::*;
use std::f64;
use models::vector::Vector;

pub const BACKUP: f64 = 15.0;
pub const PUG_SIZE: f64 = 75.0;
pub const VEL: f64 = 250.0;

pub struct Pug {
    pos: Vector,
    desired_pos: Vector,
    vel: Vector,
    rotation: f64,
    texture: Result<Texture, String>,
    crashed: bool,
}


impl Pug {

    pub fn new(x: f64, y: f64 ) -> Self {
        Pug {
            pos: Vector::new(x, y),
            desired_pos: Vector::new(0.0, 0.0),
            vel: Vector::new(0.0, 0.0),
            rotation: 0.0,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("pug.png")),
            crashed: false,
        }
    }

    pub fn desired_update(&mut self, desx: f64, desy: f64) {
        self.desired_pos.x = desx;
        self.desired_pos.y = desy;

        let xdiff = desx - self.pos.x;
        let ydiff = desy - self.pos.y;
        let mag = (xdiff.powi(2) + ydiff.powi(2)).sqrt();
        let unitx = xdiff / mag;
        let unity = ydiff / mag;

        self.rotation = unity.atan2(unitx);
    }

    fn mov(&mut self, width: f64, height: f64) {

        if self.pos.x < 0.0 {
            self.pos.x += BACKUP;
        } else if self.pos.x > width {
            self.pos.x -= BACKUP;
        }

        if self.pos.y < 0.0 {
            self.pos.y += BACKUP;
        } else if self.pos.y > height {
            self.pos.y -= BACKUP;
        }

        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }

    pub fn update(&mut self, args: &UpdateArgs, desx: f64, desy: f64, dimensions: &[f64; 2]) {

        self.desired_update(desx, desy);
        let dist = self.pos.dist(&self.desired_pos);

        if dist > PUG_SIZE+25.0 {
            self.vel.x = VEL * args.dt * self.rotation.cos();
            self.vel.y = VEL * args.dt * self.rotation.sin();
        } else {
            self.crashed = true;
            self.vel.reset();
        }

        self.mov(dimensions[0], dimensions[1]);

    }

    pub fn draw(&self, c: self::graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;

        let transform = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-PUG_SIZE / 2.0, -PUG_SIZE / 2.0);

        match self.texture {
            Ok(ref t) => image(t, transform, gl),
            _ => {}
        }
    }

    pub fn get_x(&self) -> f64 {
        self.pos.x
    }

    pub fn get_y(&self) -> f64 {
        self.pos.y
    }

    pub fn get_crashed (&self) -> bool {
        self.crashed
    }


}