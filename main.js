const wasmStuff = require('./pkg/gameoflife_wasm')

const game = wasmStuff.createGame(50)

console.log(game.grid)