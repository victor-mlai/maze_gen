use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

//use queues::*;
//use std::collections::VecDeque;
use crate::grid;

/// Pseudocode source: https://en.wikipedia.org/wiki/Maze_generation_algorithm

pub trait Observer {
    fn update(&self);
}

struct FakeObserver;
impl Observer for FakeObserver {
    fn update(&self) {}
}

//mod maze_gen;
//
//trait Algo {
//    fn generate() -> grid::Grid;
//}

/// Kruskal's Algorithm: creates a random maze by repeatedly adding random walls between cells,
/// then merging sets of cells when a wall is removed that connects them.
/// This continues until all cells are in the same set, meaning they are all connected.
pub fn create_rand_kruskal() {
    todo!()
}

/// Prim's Algorithm: starts by adding a random cell to a set, then repeatedly chooses a random frontier cell to this set's cells and creates a passage between them.
/// This continues until all cells are in the set.
pub fn create_rand_prim(
    nr_columns: u32,
    obs: Option<&dyn Observer>,
    seed: Option<u64>,
) -> grid::Grid {
    let fake_obs = FakeObserver {};
    let obs = obs.unwrap_or(&fake_obs);

    let grid_size = nr_columns * 2 + 1;
    // 1. Start with a grid full of walls.
    let mut ret = grid::Grid::new(grid_size);

    let mut gen = StdRng::seed_from_u64(seed.unwrap_or(rand::thread_rng().next_u64()));
    let distr = Uniform::<u8>::from(0..4);

    for i in (1..grid_size).step_by(2) {
        for j in (1..grid_size).step_by(2) {
            let (off_i, off_j) = match distr.sample(&mut gen) {
                0 => (i - 1, j),
                1 => (i + 1, j),
                2 => (i, j - 1),
                _ => (i, j + 1),
            };

            *ret.cells.get_mut((i * grid_size + j) as usize).unwrap() = grid::Cell::Empty;
            //*ret.cells
            //    .get_mut((off_i * grid_size + off_j) as usize)
            //    .unwrap() = grid::Cell::Empty;
        }
    }

    //let mut queue: Queue<(u32, u32)> = Queue::new();//vec![(0u32, 0u32); (grid_size * grid_size) as usize];
    //
    //// 2. Pick a random cell, mark it as part of the maze.
    //let rand_x = distr.sample(&mut gen);
    //let rand_y = distr.sample(&mut gen);
    //queue.add((rand_x, rand_y)).unwrap();
    //
    //// 3. While there are unvisited blocks in the list, Pick a random wall from the list.
    //while let Ok((pos_x, pos_y)) = queue.remove() {
    ////      1.  Add all unvisited neighboring blocks to the queue
    //
    //}
    //          1. Make the wall a passage and mark the unvisited cell as part of the maze.
    //          2. Add the neighboring walls of the cell to the wall list.
    //      2. Remove the wall from the list.

    ret
}
