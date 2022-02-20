use crate::cell_state::CellState;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cell {
    state: CellState
}

impl Cell {

    pub fn new(init_state: CellState) -> Cell {
        Cell { state: init_state }
    }

    pub fn get_state(&self) -> CellState {
        self.state
    }

    pub fn update_state(&mut self, no_of_live_neighbours: u8){
        if no_of_live_neighbours == 3 {
            self.state = CellState::ALIVE;
        }
        if no_of_live_neighbours < 2 || no_of_live_neighbours > 3 {
            self.state = CellState::DEAD;
        }
    }

    pub fn create_random_cell() -> Cell {
        let num: u8 = rand::thread_rng().gen_range(0..2);
        let this_cell_state = match num {
            0 => CellState::DEAD,
            1 => CellState::ALIVE,
            _ => panic!("Something has gone wrong.")
        };
        return Cell::new(this_cell_state);
    }
}

#[test]
fn should_create_random_cell() {
    let cell = Cell::create_random_cell();
    assert!(matches!(cell.get_state(), CellState::ALIVE | CellState::DEAD));
}

mod live_cell_should {
    use super::{Cell, CellState};

    #[test]
    fn be_instantiated_with_live_cell_state(){
        let cell = Cell {
            state: CellState::ALIVE,
        };
        assert!(matches!(cell.get_state(), CellState::ALIVE));
    }

    #[test]
    fn die_when_has_fewer_than_two_live_neighbours(){
        let mut cell = Cell {
            state: CellState::ALIVE
        };
        cell.update_state(1);
        assert!(matches!(cell.get_state(), CellState::DEAD));
    }

    #[test]
    fn survive_with_two_live_neighbours(){
        let mut cell = Cell {
            state: CellState::ALIVE
        };
        cell.update_state(2);
        assert!(matches!(cell.get_state(), CellState::ALIVE));
    }

    #[test]
    fn survive_with_three_live_neighbours(){
        let mut cell = Cell {
            state: CellState::ALIVE
        };
        cell.update_state(3);
        assert!(matches!(cell.get_state(), CellState::ALIVE));
    }

    #[test]
    fn die_when_has_more_than_3_neighbours(){
        let mut cell = Cell {
            state: CellState::ALIVE
        };
        cell.update_state(4);
        assert!(matches!(cell.get_state(), CellState::DEAD));
    }
}

mod dead_cell_should {
    use super::{Cell, CellState};

    #[test]
    fn be_revived_when_has_exactly_three_live_neighbours(){
        let mut cell = Cell {
            state: CellState::DEAD
        };
        cell.update_state(3);
        assert!(matches!(cell.get_state(), CellState::ALIVE));
    }
}
