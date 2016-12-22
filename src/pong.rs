extern crate piston_window;
use piston_window::*;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

use paddle::Paddle;
use ball::Ball;
use util::*;

pub struct Pong {
    gl: GlGraphics,
    ball: Ball,
    screen_width: u32,
    screen_height: u32,
    p1_paddle: Paddle,
    p2_paddle: Paddle,
    p1_score: u8,
    p2_score: u8,
}

impl Pong {
    pub fn new(gl: GlGraphics, w: u32, h: u32) -> Self {
        let p1_point = Point::new(10, h/2);
        let p2_point = Point::new(w-10, h/2);

        Pong {
            gl: gl,
            ball: Ball::new(w, h),
            screen_width: w,
            screen_height: h,
            p1_paddle: Paddle::new(p1_point),
            p2_paddle: Paddle::new(p2_point),
            p1_score: 0,
            p2_score: 0,
        }
    }

    pub fn render(&mut self, args:&RenderArgs) {
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }
}
