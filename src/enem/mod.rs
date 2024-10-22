
#[derive(Copy, Clone, PartialEq)]
pub struct Enem {
    pub position_x: usize,
    pub position_y: usize,
}

impl Enem {
    pub fn new(x: usize, y: usize) -> Self {
        Enem {
            position_x: x,
            position_y: y,
        }
    }
}