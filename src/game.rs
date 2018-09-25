extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate find_folder;

use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::GlGraphics;
use self::rand::Rng;
use glutin_window::GlutinWindow as Window;

use models::player::Player;
use models::pug::Pug;

pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const BACK_COLOR: [f32; 4] = [2.0, 0.0, 0.0, 0.5];
pub const FPS: u64 = 60;

pub struct Game {
    player: Player,
    pugs: Vec<Pug>,
    dimensions: [f64; 2],
    game_over: bool,
    score: u32,
    back_size: [f64; 4],
}

impl Game {

	pub fn new(width:f64, height: f64) -> Self {
		Game {
			player: Player::new(width / 2.0, height / 2.0),
			pugs: Vec::<Pug>::new(),
			dimensions: [height, width],
			game_over: false,
            score: 0,
            back_size: [10.0, 10.0, width-20.0, height-20.0]
		}
	}

	fn on_draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
        use self::graphics::*;
        gl.draw(args.viewport(), |c, gl| {

            clear(WHITE, gl);

            rectangle(BACK_COLOR, self.back_size, c.transform, gl);

            //TODO on resize have player drawn to middle of screen
            self.player.draw(c, gl);

            for pug in &mut self.pugs {
                pug.draw(c, gl);
                if pug.get_crashed() {
                    text(BLACK, 72, format!("SHE WOKE").as_str(), glyph_cache, 
                        c.transform.trans(225.0,250.0), gl);
                    text(BLACK, 72, format!("Click R to Restart").as_str(), glyph_cache, 
                        c.transform.trans(125.0,350.0), gl);
                    self.game_over = true;
                    break;
                }
            }

            text(BLACK, 38, format!("Score:{}", self.score).as_str(), 
                glyph_cache, 
                c.transform.trans(15.0,55.0),
                gl);
        });
    }

    fn on_update(&mut self, args: &UpdateArgs, cursor:[f64;2]) {

        let mut indices_to_remove = Vec::new();
        let i:u32 = 0;

        for pug in &mut self.pugs {

            let xdiff = cursor[0] - pug.get_x();
            let ydiff = cursor[1] - pug.get_y();
            let dist = (xdiff.powi(2) + ydiff.powi(2)).sqrt();

            if dist < self.player.get_size() / 2.0 {
                indices_to_remove.push(i);
            }

            pug.update(args, self.player.get_x(), self.player.get_y(), &self.dimensions);
        }

        for ind in indices_to_remove {
            self.score +=1;
            if self.score % 9 == 0 {
                self.add_pug();
            }
            self.pugs.remove(ind as usize);
            self.add_pug();
        }
    }

	pub fn run(&mut self, window: &mut Window,
               mut gl: &mut GlGraphics,
               mut glyph_cache: &mut GlyphCache) {
		let mut events = Events::new(EventSettings::new());
        events.set_ups(FPS);
        self.add_pug();
        let mut cursor = [0.0, 0.0];
        while let Some(e) = events.next(window) {
            if !self.get_game_over() {

                e.mouse_cursor(|x, y| {
                    cursor = [x, y];
                });

                if let Some(r) = e.resize_args() {
                    self.on_resize(&r);
                }
                if let Some(r) = e.update_args() {
                    self.on_update(&r, cursor);
                }
            };

            if let Some(k) = e.press_args() {
                match k {
                    Button::Keyboard(key) => self.on_key_press(key),
                    _ => {}
                }
            }

            if let Some(u) = e.render_args() {
                self.on_draw(&u, &mut gl, &mut glyph_cache);
            }
        }

	}

	fn on_resize(&mut self, new_dimensions: &[u32; 2]) {
        self.dimensions[0] = new_dimensions[0] as f64;
        self.dimensions[1] = new_dimensions[1] as f64;
    }

    fn get_game_over(&mut self) -> bool {
        self.game_over
    }

    fn add_pug(&mut self) {

        let random_num = rand::thread_rng().gen_range(1, 3);

        let mut x: f64 = 0.0;
        let mut y:f64 = 0.0;
        if random_num == 1 {
            x = rand::thread_rng().gen_range(0.0, self.dimensions[0]);
            y = 0.0;
        }
        if random_num == 2 {
            x = rand::thread_rng().gen_range(0.0, self.dimensions[0]);
            y = self.dimensions[1];
        } 

        self.pugs.push(Pug::new(x, y));
    }

    fn reset(&mut self) {
        self.pugs.clear();
        self.score = 0;
        self.player.reset(self.dimensions[0], self.dimensions[1]);
        self.add_pug();
        self.game_over = false;
    }

    fn on_key_press(&mut self, key: Key) {
        match key {
            Key::R => {
                self.reset();
            }
            _ => {}
        }
    }

}