use hitbox::Hitbox;
use nalgebra::Point2;

pub struct Paddle {
    pub center: Point2<f64>,
    height: u32,
    width: u32,
}

impl Paddle {
    pub fn new(center: Point2<f64>) -> Self {
        let h = 70;
        let w = 12;

        Paddle {
            center: center,
            height: h,
            width: w,
        }
    }

    pub fn set_location(&mut self, y: i32) {
        self.center = Point2::new(self.center.x, y as f64);
    }

    // pub fn center(&self) -> Point2<f64> {
    //     self.center
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Hitbox for Paddle {
    fn top(&self) -> i32 {
        (self.center.y - self.height() as f64 / 2.0) as i32
    }

    fn bottom(&self) -> i32 {
        (self.center.y + self.height() as f64 / 2.0) as i32
    }

    fn left(&self) -> i32 {
        (self.center.x - self.width() as f64 / 2.0) as i32
    }

    fn right(&self) -> i32 {
        (self.center.x + self.width() as f64 / 2.0) as i32
    }
}
