extern crate ncollide_geometry;
extern crate nalgebra;
extern crate piston;

extern crate piston_window;
use piston_window::*;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use opengl_graphics::error::Error as GlError;

extern crate sdl2_window;
use sdl2_window::Sdl2Window;

pub extern crate rand;

mod pong;
use pong::Pong;
mod controls;
mod paddle;
mod ball;
mod hitbox;
mod player;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(
        "Pong",
        [800, 600],
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .expect("Failed to create window");

    let font = GlyphCache::new("assets/NovaMono.ttf");

    let mut pong = Pong::new(
        window.size().width,
        window.size().height,
        font,
    );

    let mut gl = GlGraphics::new(opengl);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Render(args) => {
                pong.render(&mut gl, &args);
            },
            Event::Update(args) => {
                pong.update(&args);
                window.set_title(pong.title());
            },
            Event::Input(input) => {
                pong.input(&input);
            },
            _ => {},
        }
    }
}
