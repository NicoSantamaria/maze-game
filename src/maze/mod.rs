use rand::{
    seq::SliceRandom,
    thread_rng,
    Rng
};

use crate::{enem, types};

pub fn generate_maze(maze: &mut types::MazeGrid, x: usize, y: usize) {
    let directions: [(isize, isize); 4] = [(0, 2), (0, -2), (2, 0), (-2, 0)];
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let mut shuffled_directions: Vec<(isize, isize)> = directions.to_vec();
    shuffled_directions.shuffle(&mut rng);

    maze[x][y] = types::MazeTypes::None;

    for &(dx, dy) in &shuffled_directions {
        let next_x: usize = (x as isize + dx) as usize;
        let next_y: usize = (y as isize + dy) as usize;

        if {
            next_x < types::DIMENSION &&
            next_y < types::DIMENSION &&
            maze[next_x][next_y] == types::MazeTypes::Wall
        } {
            let x_coord: usize = (x as isize + dx / 2) as usize;
            let y_coord: usize = (y as isize + dy / 2) as usize;
            maze[x_coord][y_coord] = types::MazeTypes::None;

            generate_maze(maze, next_x, next_y);
        }
    }
}

pub fn generate_enems(maze: &types::MazeGrid, num: &u32) -> Vec<enem::Enem> {
    let mut enems: Vec<enem::Enem> = vec![];
    let min: usize = types::DIMENSION / 2;
    let max: usize = types::DIMENSION;

    for _ in 0..*num {
        let mut valid: bool = false;

        while !valid {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let random1: usize = rng.gen_range(min..max);
            let random2: usize = rng.gen_range(min..max);

            match maze[random1][random2] {
                types::MazeTypes::None => {
                    enems.push(enem::Enem::new(random1, random2));
                    valid = true;
                },
                _ => {}
            };
        }
    }

    return enems;
}