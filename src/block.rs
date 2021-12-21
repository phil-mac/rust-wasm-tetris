use crate::cell::Cell;
use crate::coord::Coord;
// use rand::Rng;

pub struct Block {
  pub coords: [Coord; 4],
  pub prev_coords: [Coord; 4],
  pub color: Cell,
}

#[derive(Clone)]
pub struct BlockTemplate {
  starting_coords: [Coord; 4],
  color: Cell,
}

fn null_coords() -> [Coord; 4] {
  [
    Coord { x: 0, y: 0 },
    Coord { x: 0, y: 0 },
    Coord { x: 0, y: 0 },
    Coord { x: 0, y: 1 },
  ]
}

impl Block {
  pub fn translation(&self, direction: fn(Coord) -> Coord) -> [Coord; 4] {
    let mut new_coords = null_coords();
    for index in 0..=3 {
      new_coords[index] = direction(self.coords[index]);
    }
    new_coords
  }

  pub fn rotation(&self) -> [Coord; 4] {
    let mut new_coords = null_coords();

    let x_pivot = self.coords[0].x;
    let y_pivot = self.coords[0].y;

    for index in 0..=4 {
      let x_delta = self.coords[index].x - x_pivot;
      let y_delta = self.coords[index].y - y_pivot;

      new_coords[index] = Coord {
        x: y_delta + x_pivot,
        y: x_delta + y_pivot,
      }
    }
    new_coords
  }

  pub fn new() -> Block {
    let t_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 2 },
        Coord { x: 4, y: 2 },
        Coord { x: 5, y: 1 },
        Coord { x: 6, y: 2 },
      ],
      color: Cell::Color1,
    };
    let j_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 2 },
        Coord { x: 4, y: 1 },
        Coord { x: 4, y: 2 },
        Coord { x: 6, y: 2 },
      ],
      color: Cell::Color2,
    };
    let z_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 2 },
        Coord { x: 4, y: 1 },
        Coord { x: 5, y: 1 },
        Coord { x: 6, y: 2 },
      ],
      color: Cell::Color3,
    };
    let o_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 1 },
        Coord { x: 6, y: 1 },
        Coord { x: 5, y: 2 },
        Coord { x: 6, y: 2 },
      ],
      color: Cell::Color1,
    };
    let s_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 2 },
        Coord { x: 4, y: 2 },
        Coord { x: 5, y: 1 },
        Coord { x: 6, y: 1 },
      ],
      color: Cell::Color2,
    };
    let l_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 2 },
        Coord { x: 4, y: 2 },
        Coord { x: 6, y: 2 },
        Coord { x: 6, y: 1 },
      ],
      color: Cell::Color3,
    };
    let i_block = BlockTemplate {
      starting_coords: [
        Coord { x: 5, y: 1 },
        Coord { x: 4, y: 1 },
        Coord { x: 6, y: 1 },
        Coord { x: 7, y: 1 },
      ],
      color: Cell::Color1,
    };

    let block_templates = [
      t_block, j_block, z_block, o_block, s_block, l_block, i_block,
    ];
    let rando = js_sys::Math::floor(js_sys::Math::random() * 6 as f64);
    // let random_index = rand::thread_rng().gen_range(0..6);
    let block_template = &block_templates[rando as usize];
    Block {
      coords: block_template.starting_coords,
      prev_coords: block_template.starting_coords,
      color: block_template.color.clone(),
    }
  }
}
