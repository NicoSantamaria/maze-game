use rand::{seq::SliceRandom, thread_rng};
use crate::types::{self, MazeGrid, MazeTypes};

#[derive(Copy, Clone, PartialEq)]
pub struct Enem {
    pub position_x: usize,
    pub position_y: usize,
    pub last_move: (isize, isize),
}

impl Enem {
    const MOVES: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    pub fn new(x: usize, y: usize) -> Self {
        Enem {
            position_x: x,
            position_y: y,
            last_move: Self::MOVES
                .choose(&mut thread_rng())
                .copied()
                .unwrap_or((0, 0)),
        }
    }

    /// Calculates and executes the next move for the enemy
    /// Returns the movement vector and new position
    pub fn new_move(&mut self, board: &MazeGrid) -> ((isize, isize), (usize, usize)) {
        let mut forward_moves: Vec<((isize, isize), (usize, usize))> = Vec::with_capacity(3);
        let backwards: (isize, isize) = (-self.last_move.0, -self.last_move.1);
        let mut backwards_move: Option<((isize, isize), (usize, usize))> = None;

        // First, calculate all possible moves including backwards
        for &next_move in &Self::MOVES {
            if let Some((next_x, next_y)) = self.calculate_next_position(next_move) {
                if self.is_valid_move(board, next_x, next_y) {
                    if next_move == backwards {
                        backwards_move = Some((next_move, (next_x, next_y)));
                    } else {
                        forward_moves.push((next_move, (next_x, next_y)));
                    }
                }
            }
        }

        // Choose move based on available options
        let (move_vector, new_position) = if !forward_moves.is_empty() {
            // If we have forward moves available, randomly choose one
            forward_moves
                .choose(&mut thread_rng())
                .copied()
                .unwrap()
        } else if let Some(back_move) = backwards_move {
            // If only backwards movement is possible, use that
            back_move
        } else {
            // This should never happen if the maze is properly constructed
            panic!("Enemy is completely trapped with no valid moves!");
        };

        // Update enemy position
        self.last_move = move_vector;
        // self.position_x = new_position.0;
        // self.position_y = new_position.1;

        (move_vector, new_position)
    }

    /// Calculates the next position given a move vector
    fn calculate_next_position(&self, (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
        let next_x: usize = self.position_x.checked_add_signed(dx)?;
        let next_y: usize = self.position_y.checked_add_signed(dy)?;

        if next_x < types::DIMENSION && next_y < types::DIMENSION {
            Some((next_x, next_y))
        } else {
            None
        }
    }

    /// Checks if a move to the given position is valid
    fn is_valid_move(&self, board: &MazeGrid, next_x: usize, next_y: usize) -> bool {
        match board[next_x][next_y] {
            MazeTypes::Wall => false,
            _ => true,
        }
    }
}