use nalgebra::Point2;

pub struct Ball {
    pub center: Point2<f64>,
    // status: BallStatus,
    pub visible: bool,
    size: u32,
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

    pub fn increase_frames(&mut self) {
        let (new_count, overflow) = self.frames.overflowing_add(1);
        if ! overflow {
            self.frames = new_count;
        } else {
            self.frames = 10;
        }
    }

    pub fn update_position(&mut self, dt: f64) {
        let new_ball_x = self.center.x + self.dx * dt * (self.speed as f64 - 4.0);
        let new_ball_y = self.center.y + self.dy * dt * (self.speed as f64 - 4.0);
        self.center = Point2::new(new_ball_x, new_ball_y);
    }

    pub fn increase_speed(&mut self) {
        // self.speed.saturating_add(1);
        self.speed += 1;
        println!("{}", self.speed);
        // self.dx += 50.0;
        // self.dy += 50.0;
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

// #[derive(Copy, Clone)]
// pub enum BallStatus {
//     Ready,
//     Launched,
// }
