/* vim: set et sts=4 sw=4: */

use std::collections::{BitvSet, bitv};
use std::default::Default;

mod clone;
mod default;
mod fmt;

#[derive(Clone)]
struct Cell {
    possibilities: BitvSet,
    value : Option<uint>,
}

impl Cell {
    fn new(value : &Option<uint>) -> Cell {
        Cell {
            value : *value,
            possibilities : match *value {
                Some(n) => {
                    assert!(n >= 1 && n <= 9);
                    let mut result = BitvSet::new();
                    result.insert(n);
                    result
                },
                None => BitvSet::from_bitv(bitv::from_bytes(&[0b01111111, 0b11000000]))
            },
        }
    }
}

#[derive(Default,Clone)]
pub struct Puzzle {
    cells: [[Cell; 9]; 9],
    recursion_depth : uint,
}

impl Puzzle {
    pub fn set_item(&mut self, col : uint, row : uint, val : uint) {
        assert!(self.cells[row][col].possibilities.contains(&val),
            "assertion failed: ({},{}): {} doesn't contain {}",
            col, row, self.cells[row][col].possibilities, val);
        assert_eq!(self.cells[row][col].value, None);
        assert!(1 <= val && val <= 9, "assertion failed: `(1 <= val <= 9)` (val: `{}`)", val);

        self.cells[row][col].value = Some(val);

        for i in range(0, 9) {
            self.cells[row][i].possibilities.remove(&val);
            self.cells[i][col].possibilities.remove(&val);
        }

        let col_orig = 3 * (col / 3);
        let row_orig = 3 * (row / 3);
        assert_eq!(col_orig % 3, 0);
        assert_eq!(row_orig % 3, 0);
        for row in self.cells.iter_mut().skip(row_orig).take(3) {
            for cell in row.iter_mut().skip(col_orig).take(3) {
                cell.possibilities.remove(&val);
            }
        }

        self.cells[row][col].possibilities.clear();
    }

    fn solve_select_single_possibility(&mut self) -> bool {
        let mut found_something = false;
        for row in range(0, self.cells.len()) {
            for col in range(0, self.cells[row].len()) {
                if self.cells[row][col].possibilities.len() == 1 {
                    let val = match self.cells[row][col].possibilities.iter().next() {
                        Some(n) => n,
                        None    => panic!(),
                    };
                    self.set_item(col, row, val);
                    found_something = true;
                }
            }
        }
        debug!("{}:{}:solve_select_single_possibility: found_something={}", file!(), line!(), found_something);
        found_something
    }

    fn solve_select_single_possible_location(&mut self) -> bool {
        let mut found_something = false;
        for row in range(0, self.cells.len()) {
            let mut val_counts = [0u; 9];
            for (_, cell) in self.cells[row].iter().enumerate() {
                for val in cell.possibilities.iter() {
                    val_counts[val-1] += 1;
                }
            }
            for (_, cell) in self.cells[row].iter().enumerate() {
                match cell.value {
                    Some(val) => assert_eq!(val_counts[val-1], 0),
                    None => (),
                }
            }
            for (val_idx, cnt) in val_counts.iter().enumerate() {
                if *cnt == 1 {
                    let val = val_idx + 1u;
                    for col in range(0, self.cells[row].len()) {
                        if self.cells[row][col].possibilities.contains(&val) {
                            self.set_item(col, row, val);
                            found_something = true;
                        }
                    }
                }
            }
        }

        for col in range(0, self.cells[0].len()) {
            let mut val_counts = [0u; 9];
            for row in range(0, self.cells.len()) {
                for val in self.cells[row][col].possibilities.iter() {
                    val_counts[val-1] += 1;
                }
            }
            for row in range(0, self.cells.len()) {
                match self.cells[row][col].value {
                    Some(val) => assert_eq!(val_counts[val-1], 0),
                    None => (),
                }
            }
            for (val_idx, cnt) in val_counts.iter().enumerate() {
                if *cnt == 1 {
                    let val = val_idx + 1u;
                    for row in range(0, self.cells.len()) {
                        if self.cells[row][col].possibilities.contains(&val) {
                            self.set_item(col, row, val);
                            found_something = true;
                        }
                    }
                }
            }
        }

        for row_block in range(0u, 3u) {
            for col_block in range(0u, 3u) {
                let mut val_counts = [0u; 9];

                for i in range(0u, 9u) {
                    let row = row_block * 3u + i % 3u;
                    let col = col_block * 3u + i / 3u;

                    for val in self.cells[row][col].possibilities.iter() {
                        val_counts[val-1] += 1;
                    }
                }

                for i in range(0u, 9u) {
                    let row = row_block * 3u + i % 3u;
                    let col = col_block * 3u + i / 3u;

                    match self.cells[row][col].value {
                        Some(val) => assert_eq!(val_counts[val-1], 0),
                        None => (),
                    }
                }

                for (val_idx, cnt) in val_counts.iter().enumerate() {
                    if *cnt == 1 {
                        let val = val_idx + 1u;
                        for i in range(0u, 9u) {
                            let row = row_block * 3u + i % 3u;
                            let col = col_block * 3u + i / 3u;

                            if self.cells[row][col].possibilities.contains(&val) {
                                self.set_item(col, row, val);
                                found_something = true;
                            }
                        }
                    }
                }
            }
        }

        debug!("{}:{}:solve_select_single_possible_location: found_something={}", file!(), line!(), found_something);

        found_something
    }

    fn guess(&mut self) -> bool {
        if self.is_invalid() {
            return false;
        }

        // Ensure we try to guess first in cells with the fewest possibilities (i.e. biggest chance
        // of success)
        // TODO: persist this list in Puzzle and maintain it from set_item() using sorted inserts
        let mut to_search_cells : Vec<(uint, uint)> = Vec::from_fn(81, |i| (i / 9u, i % 9u));
        to_search_cells.retain(|&(row,col)| self.cells[row][col].possibilities.len() > 1);
        to_search_cells.sort_by(|&(row_a,col_a),&(row_b,col_b)| {
            let a = (self.cells[row_a][col_a].possibilities.len(), row_a, col_a);
            let b = (self.cells[row_b][col_b].possibilities.len(), row_b, col_b);
            a.cmp(&b)
        });

        for &(row_num, col_num) in to_search_cells.iter() {
            let possibilities = self.cells[row_num][col_num].possibilities.clone();
            for possibility in possibilities.iter() {
                if self.cells[row_num][col_num].possibilities.len() == 1 {
                    self.set_item(col_num, row_num, possibility);
                    return self.solve();
                }

                let mut tmp = self.clone();
                tmp.set_item(col_num, row_num, possibility);
                tmp.recursion_depth += 1;

                info!("{}:{}:backtrack({}, {}, {} in {} [{}])", file!(), line!(),
                    col_num, row_num, possibility, possibilities, tmp.recursion_depth);

                if tmp.solve() {
                    self.clone_from(&tmp);
                    return true;
                } else {
                    self.cells[row_num][col_num].possibilities.remove(&possibility);
                }
            }
        }

        self.is_solved()
    }

    pub fn solve(&mut self) -> bool {
        loop {
            if  !self.solve_select_single_possibility()
             && !self.solve_select_single_possible_location(){
                break;
            }
        }

        if self.is_solved() {
            return true;
        }
        self.guess()
    }

    pub fn is_solved(&self) -> bool {
        for row in self.cells.iter() {
            for col in row.iter() {
                if col.value == None {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_invalid(&self) -> bool {
        for col in self.cells.iter() {
            for cell in col.iter() {
                if cell.value == None && cell.possibilities.len() == 0 {
                    return true;
                }
            }
        }
        false
    }
}

pub fn create_puzzle(inp : &str) -> Puzzle {
    let mut cur_puzzle : Puzzle = Default::default();
    for (i, c) in inp.chars().enumerate() {
        if i > 81 {
            break;
        }

        let (row_num, col_num) = (i / 9u, i % 9u);
        match c {
            '1'...'9' => cur_puzzle.set_item(col_num, row_num, (c as uint - '0' as uint)),
            _ => (),
        }
    }
    cur_puzzle
}
