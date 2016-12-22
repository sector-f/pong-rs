use util::Point;

pub struct Paddle {
    center: Point,
    height: u32,
    width: u32,
}

impl Paddle {
    pub fn new(center: Point) -> Self {
        Paddle {
            center: center,
            height: 20,
            width: 5,
        }
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
