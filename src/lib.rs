use cell_state::CellState;
use wasm_bindgen::prelude::*;
use cell::Cell as GameCell;

mod cell_state;
mod cell;
pub mod game;

/* pub fn createGame(size: usize) -> JsValue {


} */

#[wasm_bindgen]
pub fn getExampleCellState() -> JsValue {
    let to_js_attempt = JsValue::from_serde(&CellState::ALIVE);

    return match to_js_attempt {
        Ok(to_js_attempt) => to_js_attempt,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}

#[wasm_bindgen]
pub fn getExampleCell() -> JsValue {
    let to_js_attempt = 
        JsValue::from_serde(&GameCell::new(CellState::ALIVE));

    return match to_js_attempt {
        Ok(to_js_attempt) => to_js_attempt,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}

#[wasm_bindgen]
pub fn getExampleCellVec() -> JsValue {

    let mut vec = vec![
        GameCell::new(CellState::ALIVE),
        GameCell::new(CellState::DEAD)
        ];
    let to_js_attempt = JsValue::from_serde(&vec);

    return match to_js_attempt {
        Ok(to_js_attempt) => to_js_attempt,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}

#[wasm_bindgen]
pub fn getExampleCellVec2() -> JsValue {

    let mut vec = Vec::new();
    for _i in 0..10 {
        vec.push(GameCell::create_random_cell())
    }
    let to_js_attempt = JsValue::from_serde(&vec);

    return match to_js_attempt {
        Ok(to_js_attempt) => to_js_attempt,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}

#[wasm_bindgen]
pub fn getExampleCellVecVec() -> JsValue {
    let mut vec = vec![vec![
        GameCell::new(CellState::DEAD), 
        GameCell::new(CellState::ALIVE),
    ],
    vec![
        GameCell::new(CellState::DEAD),
        GameCell::new(CellState::ALIVE),
    ]];

    let to_js_attempt = JsValue::from_serde(&vec);

    return match to_js_attempt {
        Ok(to_js_attempt) => to_js_attempt,
        Err(err) => JsValue::from_str(&err.to_string()),
    }
}
/* 
#[wasm_bindgen]
pub fn getUpdatedGame(game: Game) -> JsValue {
    JsValue::from_serde(&Game::update_game_state(game)).unwrap()
} */