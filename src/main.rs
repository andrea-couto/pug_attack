extern crate piston;
extern crate opengl_graphics;
extern crate glutin_window;

use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

mod game;
mod models;

pub const W_HEIGHT: f64 = 600.0;
pub const W_WIDTH: f64 = 600.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Pug Attack",
                                                 [W_WIDTH as u32, W_HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .expect("Error unwrapping window");

    let mut gl = GlGraphics::new(opengl);
    let mut g = game::Game::new(W_WIDTH, W_HEIGHT);
    let mut glyph_cache = GlyphCache::new("assets/Amatic-Bold.ttf").expect("Error unwraping fonts");
    g.run(&mut window, &mut gl, &mut glyph_cache);
}
