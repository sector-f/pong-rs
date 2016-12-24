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
            width: 10,
        }
    }

    pub fn set_location(&mut self, y: u32) {
        self.center = Point::new(self.center.x, y as f64);
    }

    pub fn top(&self) -> u32 {
        (self.center.y - self.height() as f64 / 2.0) as u32
    }

    pub fn bottom(&self) -> u32 {
        (self.center.y + self.height() as f64 / 2.0) as u32
    }

    pub fn left(&self) -> u32 {
        (self.center.x - self.width() as f64 / 2.0) as u32
    }

    pub fn right(&self) -> u32 {
        (self.center.x + self.width() as f64 / 2.0) as u32
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
