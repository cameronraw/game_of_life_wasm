use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum CellState{
    ALIVE,
    DEAD    
}