use crate::cell_state::CellState;

pub struct Cell {
    state: CellState
}

impl Cell {
    fn get_state(&self) -> CellState {
        self.state
    }

    fn update_state(&mut self, no_of_live_neighbours: u8){
        if no_of_live_neighbours == 3 {
            self.state = CellState::ALIVE;
        }
        if no_of_live_neighbours < 2 || no_of_live_neighbours > 3 {
            self.state = CellState::DEAD;
        }
    }
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
