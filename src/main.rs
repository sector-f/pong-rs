extern crate piston;

extern crate piston_window;
use piston_window::*;

extern crate graphics;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

extern crate sdl2_window;
use sdl2_window::Sdl2Window;

mod pong;
use pong::Pong;
mod paddle;
mod ball;
mod util;

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

    let mut pong = Pong::new(
        GlGraphics::new(opengl),
        window.size().width,
        window.size().height,
    );

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Render(args) => {
                pong.render(&mut GlGraphics::new(opengl), &args);
            },
            Event::Update(args) => {
                pong.update(&args);
            },
            _ => {},
        }

        // if let Some(r) = e.render_args() {
        //     pong.render(&r);
        // }

        // if let Some(u) = e.update_args() {
        //     pong.update(&u);
        // }
    }
}
