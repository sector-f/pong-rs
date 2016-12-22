use util::Point;

pub struct Ball {
    center: Point,
    status: BallStatus,
    size: u32,
    dx: i32,
    dy: i32,
    speed: u32,
    bounce: bool,
}

impl Ball {
    pub fn new(w: u32, h: u32) -> Self {
        Ball {
            center: Point::new(w/2, h/2),
            status: BallStatus::Ready,
            size: 3,
            dx: 0,
            dy: 0,
            speed: 0,
            bounce: false,
        }
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

enum BallStatus {
    Ready,
    Launched,
}
