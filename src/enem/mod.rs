use rand::{
    seq::SliceRandom,
    thread_rng,
};

#[derive(Copy, Clone, PartialEq)]
pub struct Enem {
    pub position_x: usize,
    pub position_y: usize,
    pub last_move: (isize, isize),
}

impl Enem {
    pub fn new(x: usize, y: usize) -> Self {
        Enem {
            position_x: x,
            position_y: y,
            last_move: Enem::new_move()
        }
    }

    pub fn new_move() -> (isize, isize) {
        let moves: Vec<(isize,isize)> = vec![(1,0), (-1, 0), (0, 1), (0, -1)];
        if let Some(choice) = moves.choose(&mut thread_rng()) {
            return *choice;
        } else {
            return (0, 0);
        }
    }
}