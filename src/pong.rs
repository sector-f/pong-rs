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
    ball: Ball,
    paddle_gap: u32,
    screen_width: u32,
    screen_height: u32,
    p1_paddle: Paddle,
    p2_paddle: Paddle,
    p1_score: u8,
    p2_score: u8,
}

impl Pong {
    pub fn new(w: u32, h: u32) -> Self {
        let paddle_gap = 20;

        let p1_point = Point::new(paddle_gap as f64, h as f64/2f64);
        let p2_point = Point::new(w as f64 - paddle_gap as f64, h as f64/2f64);

        Pong {
            ball: Ball::new(w as f64, h as f64),
            paddle_gap: paddle_gap,
            screen_width: w,
            screen_height: h,
            p1_paddle: Paddle::new(p1_point),
            p2_paddle: Paddle::new(p2_point),
            p1_score: 0,
            p2_score: 0,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args:&RenderArgs) {
        let mut paddle = rectangle::Rectangle::new(WHITE);
        let mut ball = rectangle::Rectangle::new(WHITE);
        // let mut ball = circle_arc::CircleArc::new(WHITE, self.ball.size() as f64, 0f64, 6.28318f64).resolution(1000);

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // Draw P1's paddle
            &paddle.draw(
                [
                    (self.p1_paddle.center().x as f64 - self.p1_paddle.width() as f64 / 2f64) as f64,
                    (self.p1_paddle.center().y as f64 - self.p1_paddle.height() as f64 / 2f64) as f64,
                    self.p1_paddle.width() as f64,
                    self.p1_paddle.height() as f64,
                ],
                &c.draw_state,
                c.transform,
                gl,
            );

            // Draw P2's paddle
            &paddle.draw(
                [
                    (self.screen_width as f64 - self.paddle_gap as f64 - self.p2_paddle.width() as f64 / 2f64) as f64,
                    (self.screen_height as f64 / 2f64 - self.p2_paddle.height() as f64 / 2.0) as f64,
                    self.p2_paddle.width() as f64,
                    self.p2_paddle.height() as f64,
                ],
                &c.draw_state,
                c.transform,
                gl,
            );

            // Draw the ball
            &ball.draw(
                [
                    (self.ball.center().x as f64 - self.ball.size() as f64 ) as f64,
                    (self.ball.center().y as f64 - self.ball.size() as f64) as f64,
                    self.ball.size() as f64,
                    self.ball.size() as f64,
                ],
                &c.draw_state,
                c.transform,
                gl,
            );
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }
}
