extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;

use opengl_graphics::GlGraphics;
//use piston::input::*;
use std::f64;
use models::vector::Vector;
use opengl_graphics::Texture;
use std::result::Result;
use self::graphics::*;
use self::graphics::rectangle::square;


const PLAYER_SIZE: f64 = 150.0;

pub struct Player {
    pos: Vector,
    health: u32,
    texture: Result<Texture, String>,
}

impl Player {
    pub fn new(xpos: f64, ypos: f64) -> Self {
        Player {
            pos: Vector::new(xpos, ypos),
            health: 100,
            texture: Texture::from_path(find_folder::Search::ParentsThenKids(3, 3)
                .for_folder("assets")
                .unwrap()
                .join("player.png")),
        }
    }

    pub fn draw(&self, c: graphics::Context, gl: &mut GlGraphics) {

        // transform matrix
        let bodytrans = c.transform
            .trans(self.pos.x, self.pos.y)
            .trans(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0);

        match self.texture {
            Ok(ref t) => image(t, bodytrans, gl),
            _ => {}
        }
    }

    pub fn reset(&mut self, width: f64, height: f64) {
        self.pos.x = width / 2.0;
        self.pos.y = height / 2.0;
        self.health = 50;
    }

    pub fn get_x(&self) -> f64 {
        self.pos.x
    }

    pub fn get_y(&self) -> f64 {
        self.pos.y
    }

    pub fn get_size(&self) -> f64 {
        PLAYER_SIZE
    }

}