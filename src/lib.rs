mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Off = 0,
    On = 1,
}

#[wasm_bindgen]
pub struct Block {
    coords: [(u32, u32); 4],
    prev_coords: [(u32, u32); 4],
}

#[wasm_bindgen]
impl Block {
    pub fn move_left(&mut self) {
        for index in 0..=3 {
            self.prev_coords[index] = self.coords[index];
            self.coords[index] = (self.coords[index].0, self.coords[index].1 - 1);
        }
    }
    pub fn move_right(&mut self) {
        for index in 0..=3 {
            self.prev_coords[index] = self.coords[index];
            self.coords[index] = (self.coords[index].0, self.coords[index].1 + 1);
        }
    }
    pub fn move_down(&mut self) {
        for index in 0..=3 {
            self.prev_coords[index] = self.coords[index];
            self.coords[index] = (self.coords[index].0 + 1, self.coords[index].1);
        }
    }
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    block: Block,
}

impl Board {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Board {
    pub fn tick(&mut self) {
        //move block down by one
        self.block.move_down();

        self.update_cells()
    }

    pub fn update_cells(&mut self) {
        let mut next = self.cells.clone();
        for coord in self.block.prev_coords {
            let index = self.get_index(coord.0, coord.1);
            next[index] = Cell::Off;
        }
        for coord in self.block.coords {
            let index = self.get_index(coord.0, coord.1);
            next[index] = Cell::On;
        }

        self.cells = next;
    }

    pub fn new() -> Board {
        let width = 10;
        let height = 20;

        let block = Block {
            coords: [(4, 4), (4, 5), (4, 6), (4, 7)],
            prev_coords: [(4, 4), (4, 5), (4, 6), (4, 7)],
        };

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Off
                } else {
                    Cell::Off
                }
            })
            .collect();

        Board {
            width,
            height,
            cells,
            block,
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

    pub fn move_block_left(&mut self) {
        self.block.move_left();
    }
    pub fn move_block_right(&mut self) {
        self.block.move_right();
    }
}
