pub extern crate rand;
use pong::rand::Rng;

extern crate piston_window;
use piston_window::*;

extern crate nalgebra;
// use ncollide_geometry::bounding_volume::AABB;
// use ncollide_geometry::bounding_volume::BoundingVolume;
use ncollide_geometry::shape::Segment;
use ncollide_geometry::query::RayCast;
use ncollide_geometry::query::ray_internal::Ray;

extern crate opengl_graphics;
use opengl_graphics::GlGraphics;

use player::*;
use paddle::Paddle;
use ball::Ball;
use hitbox::Hitbox;
use nalgebra::{Point2, Vector2};

use std::f64;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Pong {
    state: GameState,
    prevpoint: Option<PlayerNum>,
    ball: Ball,
    // left_wall: Segment<Point2<f64>>,
    // right_wall: Segment<Point2<f64>>,
    top_wall: Segment<Point2<f64>>,
    bottom_wall: Segment<Point2<f64>>,
    screen_width: u32,
    screen_height: u32,
    p1: Player,
    p2: Player,
    mouse_pos: Option<f64>,
    max_score: u32,
}

impl Pong {
    pub fn new(w: u32, h: u32) -> Self {
        let paddle_gap = 30;

        let p1_point = Point2::new(paddle_gap as f64, h as f64 / 2.0);
        let p2_point = Point2::new(w as f64 - paddle_gap as f64, h as f64 / 2.0);

        let player1 = Player {
                number: PlayerNum::P1,
                paddle: Paddle::new(p1_point),
                playertype: PlayerType::Human(InputMethod::Mouse),
                score: 0,
        };

        // let p1_paddle = Paddle::new(p1_point);
        // let p1_paddle_height = p1_paddle.height();

        let p2_paddle = Paddle::new(p2_point);
        let p2_paddle_height = p2_paddle.height();

        // let player1 = Player {
        //         number: PlayerNum::P1,
        //         paddle: p1_paddle,
        //         playertype: PlayerType::CPU(AI::new(p1_paddle_height, 4.0)),
        //         score: 0,
        // };

        let player2 = Player {
                number: PlayerNum::P2,
                paddle: p2_paddle,
                playertype: PlayerType::CPU(AI::new(p2_paddle_height, 3.0)),
                score: 0,
        };

        Pong {
            state: GameState::Unstarted,
            prevpoint: None,
            ball: Ball::new(w as f64, h as f64),
            screen_width: w,
            screen_height: h,
            // left_wall: Segment::new(
            //     Point2::new(0.0, 0.0),
            //     Point2::new(0.0, h as f64),
            // ),
            // right_wall: Segment::new(
            //     Point2::new(w as f64, 0.0),
            //     Point2::new(w as f64, h as f64),
            // ),
            top_wall: Segment::new(
                Point2::new(0.0, 0.0),
                Point2::new(w as f64, 0.0),
            ),
            bottom_wall: Segment::new(
                Point2::new(0.0, h as f64),
                Point2::new(w as f64, h as f64),
            ),
            p1: player1,
            p2: player2,
            mouse_pos: None,
            max_score: 10,
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args:&RenderArgs) {
        let paddle = rectangle::Rectangle::new(WHITE);
        let center = rectangle::Rectangle::new(WHITE);
        let ball = rectangle::Rectangle::new(WHITE);

        gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // Draw center line
            for y in (0..self.screen_height).filter(|i| i % 40 == 0) {
                &center.draw(
                    [
                        self.screen_width as f64 / 2.0 - 1.0,
                        (y + 10) as f64,
                        2.0,
                        20.0,
                    ],
                    &c.draw_state,
                    c.transform,
                    gl,
                );
            }

            // Draw P1's paddle
            &paddle.draw(
                [
                    self.p1.paddle.left() as f64,
                    self.p1.paddle.top() as f64,
                    self.p1.paddle.width() as f64,
                    self.p1.paddle.height() as f64,
                ],
                &c.draw_state,
                c.transform,
                gl,
            );

            // Draw P2's paddle
            &paddle.draw(
                [
                    self.p2.paddle.left() as f64,
                    self.p2.paddle.top() as f64,
                    self.p2.paddle.width() as f64,
                    self.p2.paddle.height() as f64,
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
            },
            GameState::Started => {
                self.ball.center.x += self.ball.dx * args.dt;
                self.ball.center.y += self.ball.dy * args.dt;

                let p1_paddle_hitbox = self.p1.paddle.aabb();
                let p2_paddle_hitbox = self.p2.paddle.aabb();

                // Collision detection
                let ball_top_right_ray = Ray {
                    origin: self.ball.top_right(),
                    dir: Vector2::new(self.ball.dx, self.ball.dy),
                };

                let ball_top_left_ray = Ray {
                    origin: self.ball.top_left(),
                    dir: Vector2::new(self.ball.dx, self.ball.dy),
                };

                let ball_bottom_right_ray = Ray {
                    origin: self.ball.bottom_right(),
                    dir: Vector2::new(self.ball.dx, self.ball.dy),
                };

                let ball_bottom_left_ray = Ray {
                    origin: self.ball.bottom_left(),
                    dir: Vector2::new(self.ball.dx, self.ball.dy),
                };

                // Check for collision with top wall
                if let Some(scalar) =
                    self.top_wall.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_top_left_ray,
                        true,
                    ) {
                        if scalar - 0.025 <= 0.0 {
                            self.ball.dy *= -1.0;
                        }
                } else if let Some(scalar) =
                    self.top_wall.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_top_right_ray,
                        true,
                    ) {
                        if scalar - 0.025 <= 0.0 {
                            self.ball.dy *= -1.0;
                        }
                }

                // Check for collision with bottom wall
                if let Some(scalar) =
                    self.bottom_wall.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_bottom_left_ray,
                        true,
                    ) {
                        if scalar - 0.025 <= 0.0 {
                            self.ball.dy *= -1.0;
                        }
                } else if let Some(scalar) =
                    self.bottom_wall.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_bottom_right_ray,
                        true,
                    ) {
                        if scalar - 0.025 <= 0.0 {
                            self.ball.dy *= -1.0;
                        }
                }

                // To hopefully solve any remaining issues
                if self.ball.top() < 0 {
                    self.ball.center.y =
                        0.0 + self.ball.size as f64 / 2.0;
                } else if self.ball.bottom() > self.screen_height as i32 {
                    self.ball.center.y =
                        self.screen_height as f64 - self.ball.size as f64 / 2.0;
                }

                // Check for collision with p1 paddle
                let pi = f64::consts::PI;
                if let Some(scalar) =
                    p1_paddle_hitbox.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_top_left_ray,
                        true,
                    ) {
                        if scalar - 0.005 <= 0.0 {
                            let offset =
                                (self.p1.paddle.center.y - self.ball.center.y)
                                / (self.p1.paddle.height() as f64 / 2.0);
                            let angle = offset * (pi / 3.0);
                            self.ball.increase_speed();
                            self.ball.dx = self.ball.speed as f64 * angle.cos();
                            self.ball.dy = self.ball.speed as f64 * -angle.sin();

                            match self.p2.playertype {
                                PlayerType::CPU(ref mut ai) => {
                                    ai.target = random_target(self.p2.paddle.height());
                                },
                                _ => {},
                            }
                        }
                } else if let Some(scalar) =
                    p1_paddle_hitbox.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_bottom_left_ray,
                        true,
                    ) {
                        if scalar - 0.005 <= 0.0 {
                            let offset =
                                (self.p1.paddle.center.y - self.ball.center.y)
                                / (self.p1.paddle.height() as f64 / 2.0);
                            let angle = offset * (pi / 3.0);
                            self.ball.increase_speed();
                            self.ball.dx = self.ball.speed as f64 * angle.cos();
                            self.ball.dy = self.ball.speed as f64 * -angle.sin();

                            match self.p2.playertype {
                                PlayerType::CPU(ref mut ai) => {
                                    ai.target = random_target(self.p2.paddle.height());
                                },
                                _ => {},
                            }
                        }
                }

                // Check for collision with p2 paddle
                if let Some(scalar) =
                    p2_paddle_hitbox.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_top_right_ray,
                        true,
                    ) {
                        if scalar - 0.005 <= 0.0 {
                            let offset =
                                (self.p2.paddle.center.y - self.ball.center.y)
                                / (self.p2.paddle.height() as f64 / 2.0);
                            let angle = offset * (pi / 3.0);
                            self.ball.increase_speed();
                            self.ball.dx = self.ball.speed as f64 * -angle.cos();
                            self.ball.dy = self.ball.speed as f64 * -angle.sin();

                            match self.p1.playertype {
                                PlayerType::CPU(ref mut ai) => {
                                    ai.target = random_target(self.p1.paddle.height());
                                },
                                _ => {},
                            }
                        }
                } else if let Some(scalar) =
                    p2_paddle_hitbox.toi_with_ray(
                        &nalgebra::Identity::new(),
                        &ball_bottom_right_ray,
                        true,
                    ) {
                        if scalar - 0.005 <= 0.0 {
                            let offset =
                                (self.p2.paddle.center.y - self.ball.center.y)
                                / (self.p2.paddle.height() as f64 / 2.0);
                            let angle = offset * (pi / 3.0);
                            self.ball.increase_speed();
                            self.ball.dx = self.ball.speed as f64 * -angle.cos();
                            self.ball.dy = self.ball.speed as f64 * -angle.sin();

                            match self.p1.playertype {
                                PlayerType::CPU(ref mut ai) => {
                                    ai.target = random_target(self.p1.paddle.height());
                                },
                                _ => {},
                            }
                        }
                }

                // Update paddles
                match self.p1.playertype {
                    PlayerType::Human(ref method) => {
                        match method {
                            &InputMethod::Mouse => {
                                if let Some(pos) = self.mouse_pos {
                                    let half_paddle = self.p1.paddle.height() as f64 / 2.0;
                                    let center_to_top = pos - half_paddle;
                                    let center_to_bottom = pos + half_paddle;

                                    if center_to_top as f64 > 0.0 && center_to_bottom < self.screen_height as f64 {
                                        self.p1.paddle.center.y = pos;
                                    } else if pos > 0.0 && pos < half_paddle {
                                        self.p1.paddle.center.y = half_paddle;
                                    } else if pos < self.screen_height as f64 && pos > self.screen_height as f64 - half_paddle {
                                        self.p1.paddle.center.y = self.screen_height as f64 - half_paddle;
                                    }
                                }
                            },
                        }
                    },
                    PlayerType::CPU(ref ai) => {
                        let target = match ai.target {
                            Target::Center => { self.p1.paddle.center.y },
                            Target::Top(n) => { self.p1.paddle.center.y - n },
                            Target::Bottom(n) => { self.p1.paddle.center.y - n },
                        };

                        let diff = self.ball.center.y - target;
                        let dy = f64::min(diff.abs(), (ai.max_speed));
                        if self.ball.center.y < target - 0.1 {
                            if target - (self.p1.paddle.height() as f64 / 2.0) > 0.0 {
                                self.p1.paddle.center.y -= dy;
                            }
                        } else if self.ball.center.y > target + 0.1 {
                            if target + (self.p1.paddle.height() as f64 / 2.0) < self.screen_height as f64 {
                                self.p1.paddle.center.y += dy;
                            }
                        }
                    },
                }

                // Update player 2's paddle
                match self.p2.playertype {
                    PlayerType::Human(ref method) => {
                        match method {
                            &InputMethod::Mouse => {
                                if let Some(pos) = self.mouse_pos {
                                    let half_paddle = self.p2.paddle.height() as f64 / 2.0;
                                    let center_to_top = pos - half_paddle;
                                    let center_to_bottom = pos + half_paddle;

                                    if center_to_top as f64 > 0.0 && center_to_bottom < self.screen_height as f64 {
                                        self.p2.paddle.center.y = pos;
                                    } else if pos > 0.0 && pos < half_paddle {
                                        self.p2.paddle.center.y = half_paddle;
                                    } else if pos < self.screen_height as f64 && pos > self.screen_height as f64 - half_paddle {
                                        self.p2.paddle.center.y = self.screen_height as f64 - half_paddle;
                                    }
                                }
                            },
                        }
                    },
                    PlayerType::CPU(ref ai) => {
                        let target = match ai.target {
                            Target::Center => { self.p2.paddle.center.y },
                            Target::Top(n) => { self.p2.paddle.center.y - n },
                            Target::Bottom(n) => { self.p2.paddle.center.y - n },
                        };

                        let diff = self.ball.center.y - target;
                        let dy = f64::min(diff.abs(), (ai.max_speed));
                        if self.ball.center.y < target - 0.1 {
                            if self.p2.paddle.center.y - (self.p2.paddle.height() as f64 / 2.0) > 0.0 {
                                self.p2.paddle.center.y -= dy;
                            }
                        } else if self.ball.center.y > target + 0.1 {
                            if self.p2.paddle.center.y + (self.p2.paddle.height() as f64 / 2.0) < self.screen_height as f64 {
                                self.p2.paddle.center.y += dy;
                            }
                        }
                    },
                }

                // Check for a win
                if self.p1.score == self.max_score {
                    self.ball.visible = false;
                    self.state = GameState::P1Win;
                } else if self.p2.score == self.max_score {
                    self.ball.visible = false;
                    self.state = GameState::P2Win;
                }

                // See if either player scores
                // Maybe replace this with raycasting later
                if self.ball.left() <= 0 {
                    self.p2.score += 1;
                    self.prevpoint = Some(PlayerNum::P2);
                    self.start();
                } else if self.ball.right() >= self.screen_width as i32 {
                    self.p1.score += 1;
                    self.prevpoint = Some(PlayerNum::P1);
                    self.start();
                }
            },
            GameState::P1Win => {
                self.ball.visible = false;
            },
            GameState::P2Win => {
                self.ball.visible = false;
            },
            GameState::Paused => {},
        }
    }

    pub fn input(&mut self, input: &Input) {
        match input {
            &Input::Move(motion) => {
                match motion {
                    Motion::MouseCursor(_, y) => {
                        self.mouse_pos = Some(y);
                    },
                    _ => {},
                }
            },
            &Input::Release(button) => {
                match button {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Space => {
                                match self.state {
                                    GameState::Unstarted => {
                                        self.start();
                                    },
                                    GameState::Started => {
                                        self.state = GameState::Paused;
                                    },
                                    GameState::Paused => {
                                        self.state = GameState::Started;
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
    }

    pub fn start(&mut self) {
        self.state = GameState::Started;
        self.ball.center = Point2::new(
            self.screen_width as f64 / 2.0,
            self.screen_height as f64 / 2.0
        );
        self.ball.speed = 400;
        match self.prevpoint {
            Some(ref player) => {
                match player {
                    &PlayerNum::P1 => {
                        self.ball.dx = -1.0 * self.ball.speed as f64;
                    },
                    &PlayerNum::P2 => {
                        self.ball.dx = self.ball.speed as f64;
                    },
                }
            },
            None => {
                if rand::thread_rng().gen::<bool>() {
                    self.ball.dx = self.ball.speed as f64;
                } else {
                    self.ball.dx = -1.0 * self.ball.speed as f64;
                }
            },
        }
        self.ball.dy = rand::thread_rng().gen_range::<i32>(-350, 351) as f64;
    }

    pub fn title(&self) -> String {
        match self.state {
            GameState::Unstarted => {
                format!("Pong: Press space to begin")
            },
            GameState::Started => {
                format!("Pong: {}-{}", self.p1.score, self.p2.score)
            },
            GameState::P1Win => {
                format!("Pong: Player 1 wins")
            },
            GameState::P2Win => {
                format!("Pong: Player 2 wins")
            },
            GameState::Paused => {
                format!("Pong: {}-{} (Paused)", self.p1.score, self.p2.score)
            }
        }
    }
}

impl Hitbox for Pong {
    fn top(&self) -> i32 {
        0
    }

    fn bottom(&self) -> i32 {
        self.screen_height as i32
    }

    fn left(&self) -> i32 {
        0
    }

    fn right(&self) -> i32 {
        self.screen_width as i32
    }
}

#[allow(dead_code)]
enum GameState {
    Unstarted,
    Started,
    Paused,
    P1Win,
    P2Win,
}
