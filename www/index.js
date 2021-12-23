import { Board, Cell } from "rust-wasm-tetris-phil-mac";
import { memory } from "rust-wasm-tetris-phil-mac/rust_wasm_tetris_phil_mac_bg";

const CELL_SIZE = 15;
const GRID_COLOR = "#CCCCCC";
const ON_COLOR1 = "#FF0000";
const ON_COLOR2 = "#00FF00";
const ON_COLOR3 = "#0000FF";
const OFF_COLOR = "#000000";

const board = Board.new();
const width = board.width();
const height = board.height();

const lineCount = document.getElementById("line-count");
const canvas = document.getElementById("rust-wasm-tetris-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let lastTick = new Date().getTime();
const interval = 0.800 * 1000; 

const tickLoop = () => {
  let currentTime = new Date().getTime();
  if ((currentTime - lastTick) > interval) {
    lastTick = currentTime;
    board.tick();
    drawCells();
  }

  lineCount.textContent = "LINES - 00" + board.line_count();

  requestAnimationFrame(tickLoop);
}

requestAnimationFrame(tickLoop);

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= width; i++){
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}
drawGrid();


const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = board.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++){
    for (let col = 0; col < width; col++){
      const idx = getIndex(row, col);
      ctx.fillStyle = cells[idx] === Cell.Color1 ? ON_COLOR1
        : cells[idx] === Cell.Color2 ? ON_COLOR2
          : cells[idx] === Cell.Color3 ? ON_COLOR3
            : OFF_COLOR;
      
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
}

document.addEventListener('keydown', function(event) {
  if(event.keyCode == 37) { // left arrow key
    board.attempt_move_block_left();
    drawCells();
  }
  else if(event.keyCode == 39) { // right arrow key
    board.attempt_move_block_right();
    drawCells();
  }
  else if(event.keyCode == 40) { // down arrow key
    board.attempt_move_block_down();
    drawCells();
  }
  else if(event.keyCode == 88) { // x key
    board.attempt_rotate_clockwise();
    drawCells();
  }
  else if(event.keyCode == 90) { // z key
    board.attempt_rotate_counterclockwise();
    drawCells();
  }
});