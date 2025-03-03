#![allow(dead_code)]

// --------------------
// Bad Code
// --------------------
pub struct Container {
    // Bad list name
    the_list: Vec<Vec<u32>>,
}

impl Container {
    // Function doesn't reveal what it's doing
    pub fn get_them(&self) -> Vec<Vec<u32>> {
        let mut list1 = vec![];

        for x in self.the_list.clone() {
            if x[0] == 4 {
                list1.push(x);
            }
        }
        list1
    }
}

// --------------------
// Better Code
// --------------------

const FLAGGED: u32 = 4;

pub struct Game1 {
    // Good list name
    game_board: Vec<Vec<u32>>,
}

impl Game1 {
    // Better code
    pub fn get_flagged_cells(&self) -> Vec<Vec<u32>> {
        let mut flagged_cells = vec![];

        // For better management, make Cell a struct
        for cell in self.game_board.clone() {
            if cell[0] == FLAGGED {
                flagged_cells.push(cell);
            }
        }

        flagged_cells
    }
}

// --------------------
// Best code
// --------------------

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    x: u32,
    y: u32,
}

impl Cell {
    pub fn is_flagged(&self) -> bool {
        true
    }
}
pub struct Game2 {
    game_board: Vec<Cell>,
}

impl Game2 {
    pub fn get_flagged_cells(&self) -> Vec<Cell> {
        let mut flagged_cells = vec![];

        for cell in self.game_board.clone() {
            if cell.is_flagged() {
                flagged_cells.push(cell);
            }
        }

        flagged_cells
    }
}
