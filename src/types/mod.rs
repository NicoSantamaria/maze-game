use crate::{enem, play};

pub const DIMENSION: usize = 37;

#[derive(Copy, Clone, PartialEq)]
pub enum MazeTypes {
    Strt,
    Ends,
    Wall,
    Play(play::Play),
    Enem(enem::Enem),
    None
}

#[derive(PartialEq)]
pub enum Action {
    None,
    Quit,
    Move(isize, isize)
}

pub type MazeGrid = [[MazeTypes; DIMENSION]; DIMENSION];