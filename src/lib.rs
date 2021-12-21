mod block;
mod cell;
mod coord;
mod utils;
use block::Block;
use cell::Cell;
use coord::Coord;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn left(coord: Coord) -> Coord {
    Coord {
        x: coord.x - 1,
        y: coord.y,
    }
}
fn right(coord: Coord) -> Coord {
    Coord {
        x: coord.x + 1,
        y: coord.y,
    }
}
fn down(coord: Coord) -> Coord {
    Coord {
        x: coord.x,
        y: coord.y + 1,
    }
}

#[wasm_bindgen]
pub struct Board {
    line_count: u32,
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    block: Block,
    is_stuck: bool,
    is_game_over: bool,
}

impl Board {
    fn get_index(&self, coord: Coord) -> usize {
        ((coord.y - 1) * self.width + (coord.x - 1)) as usize
    }

    // fn drop_new_block(&mut self) {
    //     self.block.coords = starting_coords();
    //     self.block.prev_coords = starting_coords();
    // }

    // aka (if there are any On's in it, or out of bounds - skipping checks of current coords)
    fn is_position_allowed(&self, new_position: [Coord; 4]) -> bool {
        for index in 0..=3 {
            // not allowed if position out of bounds
            let x = new_position[index].x;
            let y = new_position[index].y;
            if x == 0 || x > self.width || y == 0 || y > self.height {
                return false;
            }

            // not allowed if position is already taken
            let new_position_index = self.get_index(new_position[index]);
            // ignore coords taken up by current block position
            // TODO - refactor
            let mut skip = false;
            for coord in self.block.coords {
                if new_position_index == self.get_index(coord) {
                    skip = true;
                }
            }
            if !skip && self.cells[new_position_index] != Cell::Off {
                return false;
            }
        }

        true
    }

    fn attempt_move(&mut self, new_position: [Coord; 4]) -> bool {
        if !self.is_position_allowed(new_position) {
            return false;
        }

        for index in 0..=3 {
            self.block.prev_coords[index] = self.block.coords[index];
            self.block.coords[index] = new_position[index];
        }
        self.update_cells();
        true
    }

    fn on_stick(&mut self) {
        for row in 1..=self.height {
            let mut row_complete = true;
            for column in 1..=self.width {
                let index = self.get_index(Coord { x: column, y: row });
                if self.cells[index] == Cell::Off {
                    row_complete = false;
                }
            }
            if row_complete == true {
                for column in 1..=(self.width / 2) {
                    let index_inner = self.get_index(Coord { x: column, y: row });
                    self.cells[index_inner] = Cell::Off;

                    let index_outer = self.get_index(Coord { x: column, y: row });
                    self.cells[index_outer] = Cell::Off;

                    log!("clear")
                }

                for row_above in (2..=row).rev() {
                    for column in 1..=self.width {
                        let index = self.get_index(Coord {
                            x: column,
                            y: row_above,
                        });
                        let index_above = self.get_index(Coord {
                            x: column,
                            y: row_above - 1,
                        });
                        self.cells[index] = self.cells[index_above];
                    }
                }

                self.line_count = self.line_count + 1;
            }
        }

        self.check_for_game_over();

        self.block = Block::new();
    }

    fn check_for_game_over(&mut self) {
        let mut is_game_over = false;
        for coord in self.block.coords {
            if coord.y == 1 {
                is_game_over = true;
            }
        }

        if is_game_over {
            self.is_game_over = true;
            log!("game over");

            for cell in &mut self.cells {
                *cell = Cell::Color1
            }
        }
    }
}

#[wasm_bindgen]
impl Board {
    pub fn tick(&mut self) {
        if self.is_game_over {
            return;
        }

        if self.is_stuck {
            self.is_stuck = false;
            self.update_cells();

            return;
        }

        self.is_stuck = !self.attempt_move_block_down();

        if self.is_stuck {
            self.on_stick();
        }
    }

    pub fn update_cells(&mut self) {
        let mut next = self.cells.clone();
        for coord in self.block.prev_coords {
            let index = self.get_index(coord);
            next[index] = Cell::Off;
        }
        for coord in self.block.coords {
            let index = self.get_index(coord);
            next[index] = self.block.color
        }

        self.cells = next;
    }

    pub fn new() -> Board {
        utils::set_panic_hook();
        let line_count = 0;
        let width = 10;
        let height = 20;

        let block = Block::new();

        let cells = (0..=width * height).map(|_i| Cell::Off).collect();

        Board {
            line_count,
            width,
            height,
            cells,
            block,
            is_stuck: true,
            is_game_over: false,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn line_count(&self) -> u32 {
        self.line_count
    }

    pub fn attempt_move_block_down(&mut self) -> bool {
        self.attempt_move(self.block.translation(down))
    }
    pub fn attempt_move_block_left(&mut self) {
        self.attempt_move(self.block.translation(left));
    }
    pub fn attempt_move_block_right(&mut self) {
        self.attempt_move(self.block.translation(right));
    }
    pub fn attempt_rotate(&mut self) {
        self.attempt_move(self.block.rotation());
    }
}
