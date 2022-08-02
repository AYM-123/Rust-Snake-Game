use std::fmt::Debug;


#[derive(PartialEq)]
pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Cell {
    pub fn background() -> Self {
        Self {
            red: 20,
            green: 20,
            blue: 20,
        }
    }

    pub fn snake() -> Self {
        Self {
            red: 97,
            green: 201,
            blue: 0,
        }
    }

    pub fn snack() -> Self {
        Self {
            red: 255,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(PartialEq)]
pub struct Block {
    pub row: i32,
    pub col: i32,
    pub color: Cell,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", (self.row, self.col))
    }
}

pub struct Snake {
    pub body: Vec<Block>,
}

impl Snake {
    pub fn init(row: i32, col: i32) -> Self {
        Self {
            body: vec![Block {
                row,
                col,
                color: Cell::snake(),
            }],
        }
    }
}
