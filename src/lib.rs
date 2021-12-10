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
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Board {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // move the piece down

        self.cells = next;
    }

    pub fn new() -> Board {
        let width = 10;
        let height = 20;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::On
                } else {
                    Cell::Off
                }
            })
            .collect();

        Board {
            width,
            height,
            cells,
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
}
