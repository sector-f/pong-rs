pub enum InputMethod {
    Mouse,
    ArrowKeys,
    WASD,
}

pub struct Controls {
    pub mouse_pos: Option<f64>,
    pub arrows: Option<Arrow>,
    pub wasd: Option<WASD>,
}

pub enum Arrow {
    Up,
    Down,
}

pub enum WASD {
    W,
    A,
}
