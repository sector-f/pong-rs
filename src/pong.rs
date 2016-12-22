extern crate piston_window;
use piston_window::*;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

use paddle::Paddle;
use ball::Ball;
use util::*;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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
        let p1_rect = rectangle::Rectangle::new(WHITE);
        let p2_rect = rectangle::Rectangle::new(WHITE);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // The code below makes the borrow checker unhappy
            // p1_rect.draw(
            //     [
            //         (self.p1_paddle.center().x - self.p1_paddle.width() / 2) as f64,
            //         (self.p1_paddle.center().y - self.p1_paddle.height() / 2) as f64,
            //         self.p1_paddle.width() as f64,
            //         self.p1_paddle.height() as f64,
            //     ],
            //     &c.draw_state,
            //     c.transform,
            //     gl,
            // );
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }
}
