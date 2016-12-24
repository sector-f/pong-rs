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
    state: GameState,
    lastpoint: Option<Player>,
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
            state: GameState::Unstarted,
            lastpoint: None,
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
                    (self.p2_paddle.center.y - self.p2_paddle.height()  as f64 / 2.0) as f64,
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
                        (self.ball.center().x as f64 - self.ball.size() as f64) as f64,
                        (self.ball.center().y as f64 - self.ball.size() as f64) as f64,
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
            },
            GameState::Started => {
                // Update the ball's position
                let new_ball_x = self.ball.center.x + self.ball.dx * args.dt;
                let new_ball_y = self.ball.center.y + self.ball.dy * args.dt;
                self.ball.center = Point::new(new_ball_x, new_ball_y);

                // See if the ball hits a wall
                if self.ball.top() == 0 || self.ball.bottom() >= self.screen_height {
                    self.ball.dy *= -1.0;
                }

                println!("{} {}", self.ball.left(), self.p1_paddle.right());

                // See if the ball hits a paddle
                if self.p2_paddle.left() - self.ball.right() <= 1
                    && self.p2_paddle.top() < self.ball.top()
                    && self.p2_paddle.bottom() > self.ball.bottom() {
                        self.ball.dx *= -1.0;
                }

                if self.ball.left() - self.p1_paddle.right() <= 1
                    && self.p1_paddle.top() < self.ball.top()
                    && self.p1_paddle.bottom() > self.ball.bottom() {
                        self.ball.dx *= -1.0;
                }

                // Check for a win
                if self.p1_score == 10 {
                    self.state = GameState::P1Win;
                } else if self.p2_score == 10 {
                    self.state = GameState::P2Win;
                }

                // See if either player scores
                if self.ball.left() <= 0 {
                    self.p2_score += 1;
                    self.lastpoint = Some(Player::P2);
                    self.start();
                } else if self.ball.right() >= self.screen_width {
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
                            self.p1_paddle.set_location(y as u32);
                            self.p2_paddle.set_location(y as u32);
                        } else if y > 0.0 && y < half_paddle {
                            self.p1_paddle.set_location(half_paddle as u32);
                            self.p2_paddle.set_location(half_paddle as u32);
                        } else if y < self.screen_height as f64 && y > self.screen_height as f64 - half_paddle {
                            self.p1_paddle.set_location((self.screen_height as f64 - half_paddle) as u32);
                            self.p2_paddle.set_location((self.screen_height as f64 - half_paddle) as u32);
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
        self.ball.center = Point::new(
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
