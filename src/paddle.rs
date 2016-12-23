use util::Point;

pub struct Paddle {
    pub center: Point,
    height: u32,
    width: u32,
}

impl Paddle {
    pub fn new(center: Point) -> Self {
        Paddle {
            center: center,
            height: 70,
            width: 7,
        }
    }

    pub fn set_location(&mut self, y: u32) {
        self.center = Point::new(self.center.x, y as f64);
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
