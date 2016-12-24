use util::Point;

pub struct Ball {
    pub center: Point,
    status: BallStatus,
    pub visible: bool,
    size: u32,
    pub dx: f64,
    pub dy: f64,
    pub speed: u32,
    bounce: bool,
}

impl Ball {
    pub fn new(w: f64, h: f64) -> Self {
        Ball {
            center: Point::new(w/2f64, h/2f64),
            status: BallStatus::Ready,
            visible: true,
            size: 15,
            dx: 0f64,
            dy: 0f64,
            speed: 0,
            bounce: false,
        }
    }

    pub fn top(&self) -> u32 {
        (self.center.y - self.size() as f64 / 2.0) as u32
    }

    pub fn bottom(&self) -> u32 {
        (self.center.y + self.size() as f64 / 2.0) as u32
    }

    pub fn left(&self) -> u32 {
        (self.center.x - self.size() as f64 / 2.0) as u32
    }

    pub fn right(&self) -> u32 {
        (self.center.x + self.size() as f64 / 2.0) as u32
    }

    pub fn status(&self) -> BallStatus {
        self.status
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Copy, Clone)]
pub enum BallStatus {
    Ready,
    Launched,
}
