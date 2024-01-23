#[derive(Clone)]
pub struct Food {
    x: i32,
    y: i32,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Food {
        Food { x, y }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
}
