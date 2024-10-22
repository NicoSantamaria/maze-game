pub struct Play {
    pub position_x: usize,
    pub position_y: usize,
}

impl Play {
    pub fn new(x: usize, y: usize) -> Self {
        Play {
            position_x: x,
            position_y: y,
        }
    }
}