use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Off = 0,
  Color1 = 1,
  Color2 = 2,
  Color3 = 3,
}
