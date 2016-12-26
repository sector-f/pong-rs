extern crate piston_window;
use piston_window::*;

use ncollide_geometry::bounding_volume::AABB;
use ncollide_geometry::bounding_volume::BoundingVolume;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

use paddle::Paddle;
use ball::Ball;
use hitbox::Hitbox;
use nalgebra::Point2;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Pong {
    state: GameState,
    lastpoint: Option<Player>,
    ball: Ball,
    screen_width: u32,
    screen_height: u32,
    p1_paddle: Paddle,
    p2_paddle: Paddle,
    p1_score: u8,
    p2_score: u8,
}

impl Pong {
    pub fn new(w: u32, h: u32) -> Self {
        let paddle_gap = 30;

        let p1_point = Point2::new(paddle_gap as f64, h as f64/2f64);
        let p2_point = Point2::new(w as f64 - paddle_gap as f64, h as f64/2f64);

        Pong {
            state: GameState::Unstarted,
            lastpoint: None,
            ball: Ball::new(w as f64, h as f64),
            screen_width: w,
            screen_height: h,
            p1_paddle: Paddle::new(p1_point),
            p2_paddle: Paddle::new(p2_point),
            p1_score: 0,
            p2_score: 0,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args:&RenderArgs) {
        let paddle = rectangle::Rectangle::new(WHITE);
        let ball = rectangle::Rectangle::new(WHITE);

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // Draw P1's paddle
            &paddle.draw(
                [
                    self.p1_paddle.left() as f64,
                    self.p1_paddle.top() as f64,
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
                    self.p2_paddle.left() as f64,
                    self.p2_paddle.top() as f64,
                    self.p2_paddle.width() as f64,
                    self.p2_paddle.height() as f64,
                ],
                &c.draw_state,
                c.transform,
                gl,
            );

            // Draw the ball
            if self.ball.visible {
                &ball.draw(
                    [
                        self.ball.left() as f64,
                        self.ball.top() as f64,
                        self.ball.size() as f64,
                        self.ball.size() as f64,
                    ],
                    &c.draw_state,
                    c.transform,
                    gl,
                );
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            GameState::Unstarted => {
                // Make the game play itself for easier testing
                self.start();
            },
            GameState::Started => {
                self.ball.update_position(args.dt);

                let ball_hitbox = self.ball.aabb();
                let p1_paddle_hitbox = self.p1_paddle.aabb();
                let p2_paddle_hitbox = self.p2_paddle.aabb();

                // See if the ball hits a wall
                if self.ball.top() <= 0 || self.ball.bottom() >= self.screen_height as i32 {
                    self.ball.dy *= -1.0;
                }

                self.ball.increase_frames();

                // See if the ball hits p1's paddle
                if self.ball.frames > 5 {
                    if ball_hitbox.intersects(&p1_paddle_hitbox)
                        && self.ball.center.x > self.p1_paddle.center.x {
                            self.ball.frames = 0;
                            self.ball.dx *= -1.0;
                            self.ball.speed = self.ball.speed.saturating_add(1);
                    }
                }

                // See if the ball hits p2's paddle
                if self.ball.frames > 5 {
                    if ball_hitbox.intersects(&p2_paddle_hitbox)
                        && self.ball.center.x < self.p2_paddle.center.x {
                            self.ball.frames = 0;
                            self.ball.dx *= -1.0;
                            self.ball.speed = self.ball.speed.saturating_add(1);
                    }
                }

                // Make the game play itself for easier testing
                self.p1_paddle.set_location(self.ball.center.y as i32);
                self.p2_paddle.set_location(self.ball.center.y as i32);

                // Check for a win
                // if self.p1_score == 10 {
                //     self.state = GameState::P1Win;
                // } else if self.p2_score == 10 {
                //     self.state = GameState::P2Win;
                // }

                // See if either player scores
                if self.ball.left() <= 0 {
                    self.p2_score += 1;
                    self.lastpoint = Some(Player::P2);
                    self.start();
                } else if self.ball.right() >= self.screen_width as i32 {
                    self.p1_score += 1;
                    self.lastpoint = Some(Player::P1);
                    self.start();
                }
            },
            GameState::P1Win => {
                self.ball.visible = false;
            },
            GameState::P2Win => {
                self.ball.visible = false;
            },
        }
    }

    pub fn input(&mut self, input: &Input) {
        match self.state {
            GameState::Unstarted => {
                match input {
                    &Input::Release(button) => {
                        match button {
                            Button::Keyboard(key) => {
                                match key {
                                    Key::Space => {
                                        self.start();
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            GameState::Started => {
                match input {
                    &Input::Release(button) => {
                        match button {
                            Button::Keyboard(key) => {
                                match key {
                                    Key::Space => {
                                        self.start();
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        match input {
            &Input::Move(motion) => {
                match motion {
                    Motion::MouseCursor(_, y) => {
                        let half_paddle = self.p1_paddle.height() as f64 / 2.0;
                        let center_to_top = y - half_paddle;
                        let center_to_bottom = y + half_paddle;

                        if center_to_top as f64 > 0.0 && center_to_bottom < self.screen_height as f64 {
                            self.p1_paddle.set_location(y as i32);
                            self.p2_paddle.set_location(y as i32);
                        } else if y > 0.0 && y < half_paddle {
                            self.p1_paddle.set_location(half_paddle as i32);
                            self.p2_paddle.set_location(half_paddle as i32);
                        } else if y < self.screen_height as f64 && y > self.screen_height as f64 - half_paddle {
                            self.p1_paddle.set_location((self.screen_height as f64 - half_paddle) as i32);
                            self.p2_paddle.set_location((self.screen_height as f64 - half_paddle) as i32);
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Started;
        self.ball.center = Point2::new(
            self.screen_width as f64 / 2.0,
            self.screen_height as f64 / 2.0
        );
        self.ball.speed = 5;
        self.ball.dx = 50.0 * self.ball.speed as f64;
        self.ball.dy = 50.0 * self.ball.speed as f64;
    }

    pub fn title(&self) -> String {
        match self.state {
            GameState::Unstarted => {
                format!("Pong: Press space to begin")
            },
            GameState::Started => {
                format!("Pong: {}-{}", self.p1_score, self.p2_score)
            },
            GameState::P1Win => {
                format!("Pong: Player 1 wins")
            },
            GameState::P2Win => {
                format!("Pong: Player 2 wins")
            },
        }
    }
}

#[allow(dead_code)]
enum GameState {
    Unstarted,
    Started,
    P1Win,
    P2Win,
}

enum Player {
    P1,
    P2,
}
