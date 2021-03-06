// use ncollide_geometry::bounding_volume::AABB;
// use ncollide_geometry::query::ray_internal::Ray;
// use nalgebra::{Point2, Vector2};
use nalgebra::Point2;
use hitbox::Hitbox;

pub struct Ball {
    pub center: Point2<f64>,
    // status: BallStatus,
    pub visible: bool,
    pub size: u32,
    pub dx: f64,
    pub dy: f64,
    pub speed: u32,
    pub frames: u32,
    // bounce: bool,
}

impl Ball {
    pub fn new(w: f64, h: f64) -> Self {
        Ball {
            center: Point2::new(w/2f64, h/2f64),
            // status: BallStatus::Ready,
            visible: true,
            size: 15,
            dx: 0f64,
            dy: 0f64,
            speed: 0,
            frames: 0,
            // bounce: false,
        }
    }

    pub fn increase_speed(&mut self) {
        self.speed = self.speed.saturating_add(50);
    }

    // pub fn top_left_ray(&self) -> Ray<Point2<f64>> {
    //     Ray {
    //         origin: self.top_left(),
    //         dir: Vector2::new(self.dx, self.dy),
    //     }
    // }

    // pub fn top_right_ray(&self) -> Ray<Point2<f64>> {
    //     Ray {
    //         origin: self.top_right(),
    //         dir: Vector2::new(self.dx, self.dy),
    //     }
    // }

    // pub fn bottom_left_ray(&self) -> Ray<Point2<f64>> {
    //     Ray {
    //         origin: self.bottom_left(),
    //         dir: Vector2::new(self.dx, self.dy),
    //     }
    // }

    // pub fn bottom_right_ray(&self) -> Ray<Point2<f64>> {
    //     Ray {
    //         origin: self.bottom_right(),
    //         dir: Vector2::new(self.dx, self.dy),
    //     }
    // }

    // pub fn increase_frames(&mut self) {
    //     let (new_count, overflow) = self.frames.overflowing_add(1);
    //     if ! overflow {
    //         self.frames = new_count;
    //     } else {
    //         self.frames = 10;
    //     }
    // }

    // pub fn update_position(&mut self, dt: f64) {
    //     let new_ball_x = self.center.x + self.dx * dt;
    //     let new_ball_y = self.center.y + self.dy * dt;
    //     self.center = Point2::new(new_ball_x, new_ball_y);
    // }

    // pub fn status(&self) -> BallStatus {
    //     self.status
    // }

    // pub fn center(&self) -> Point2<f64> {
    //     self.center
    // }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Hitbox for Ball {
    fn top(&self) -> i32 {
        (self.center.y - self.size() as f64 / 2.0) as i32
    }

    fn bottom(&self) -> i32 {
        (self.center.y + self.size() as f64 / 2.0) as i32
    }

    fn left(&self) -> i32 {
        (self.center.x - self.size() as f64 / 2.0) as i32
    }

    fn right(&self) -> i32 {
        (self.center.x + self.size() as f64 / 2.0) as i32
    }

    fn half_width(&self) -> f64 {
        self.size as f64 / 2.0
    }

    fn half_height(&self) -> f64 {
        self.size as f64 / 2.0
    }
}

// #[derive(Copy, Clone)]
// pub enum BallStatus {
//     Ready,
//     Launched,
// }
