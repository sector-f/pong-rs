extern crate rand;
use rand::Rng;
use paddle::Paddle;
use controls::*;

pub struct Player {
    pub number: PlayerNum,
    pub paddle: Paddle,
    pub playertype: PlayerType,
    pub score: u32,
}

pub enum PlayerNum {
    P1,
    P2,
}

pub enum PlayerType {
    Human(InputMethod),
    CPU(AI),
}

pub struct AI {
    pub max_speed: f64,
    pub target: Target,
}

impl AI {
    pub fn new(h: u32, speed: f64) -> Self {
        AI {
            max_speed: speed,
            target: random_target(h),
        }
    }
}

pub fn random_target(h: u32) -> Target {
    let mut rng = rand::thread_rng();
    match rng.gen_range::<u8>(0, 3) {
        0 => { Target::Center },
        1 => { Target::Top(0.0 + rng.gen_range::<f64>(0.0, h as f64 / 2.0)) },
        2 => { Target::Bottom(0.0 - rng.gen_range::<f64>(0.0, h as f64 / 2.0)) },
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum Target {
    Center,
    Top(f64),
    Bottom(f64),
}
