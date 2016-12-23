use util::Point;

pub struct Ball {
    center: Point,
    status: BallStatus,
    size: u32,
    dx: f64,
    dy: f64,
    speed: u32,
    bounce: bool,
}

impl Ball {
    pub fn new(w: f64, h: f64) -> Self {
        Ball {
            center: Point::new(w/2f64, h/2f64),
            status: BallStatus::Ready,
            size: 15,
            dx: 0f64,
            dy: 0f64,
            speed: 0,
            bounce: false,
        }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

enum BallStatus {
    Ready,
    Launched,
}
