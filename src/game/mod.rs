use serde::{Deserialize, Serialize};
use crate::{cell::Cell as GameCell, cell_state::CellState as CellState};


#[derive(Serialize, Deserialize)]
pub struct Game {
    height: usize,
    width: usize,
    grid: Vec<Vec<GameCell>>
}

impl Game {

    pub fn new(size: usize) -> Self {

        let new_grid = Game::create_grid(size);

        Self {
            height: size,
            width: size,
            grid: new_grid
        }
    }

    fn create_cell_vector(size: usize) -> Vec<GameCell> {

        let mut cell_vector = Vec::<GameCell>::new();

        for _i in 0..size {
            cell_vector.push(GameCell::create_random_cell())
        }

        cell_vector
    }

    fn create_grid(size: usize) -> Vec<Vec<GameCell>> {

        let mut new_grid = Vec::<Vec<GameCell>>::new();

        for _i in 0..size {
            new_grid.push(Game::create_cell_vector(size))
        }

        new_grid
    }
}

#[test]
fn should_return_cell_vector_of_given_size(){
    let cell_vector = Game::create_cell_vector(10);
    assert_eq!(cell_vector.len(), 10);
}

#[test]
fn should_return_cell_vector_containing_cells_with_cellstate(){
    let cell_vector = Game::create_cell_vector(10);
    for cell in cell_vector.iter() {
        assert!(matches!(cell.get_state(), CellState::ALIVE | CellState::DEAD))
    }
}

#[test]
fn should_return_a_grid_of_initialized_cells(){
    let grid = Game::create_grid(10);
    assert_eq!(grid.len(), 10);
    for row in grid.iter(){
        for cell in row.iter() {
            assert!(matches!(cell.get_state(), CellState::ALIVE | CellState::DEAD))
        }
    }
}

