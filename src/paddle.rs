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
}
