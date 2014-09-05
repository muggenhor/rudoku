/* vim: set et sts=4 sw=4: */

use std::collections::{BitvSet, bitv};
use std::default::Default;

#[deriving(Clone)]
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
                None => BitvSet::from_bitv(bitv::from_bytes([0b01111111, 0b11000000]))
            },
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new(&Default::default())
    }
}

impl Default for [Cell, ..9] {
    fn default() -> [Cell, ..9] {
        [
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
        ]
    }
}

impl Default for [[Cell, ..9], ..9] {
    fn default() -> [[Cell, ..9], ..9] {
        [
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
        ]
    }
}

impl Clone for [Cell, ..9] {
    fn clone(&self) -> [Cell, ..9] {
        let mut new : [Cell, ..9] = Default::default();
        new.clone_from(self);
        new
    }
    fn clone_from(&mut self, source: &[Cell, ..9]) {
        for (src_cell, dst_cell) in source.iter().zip(self.mut_iter()) {
            dst_cell.clone_from(src_cell);
        }
    }
}

impl Clone for [[Cell, ..9], ..9] {
    fn clone(&self) -> [[Cell, ..9], ..9] {
        let mut new : [[Cell, ..9], ..9] = Default::default();
        new.clone_from(self);
        new
    }
    fn clone_from(&mut self, source: &[[Cell, ..9], ..9]) {
        for (src_col, dst_col) in source.iter().zip(self.mut_iter()) {
            dst_col.clone_from(src_col);
        }
    }
}

#[deriving(Default,Clone)]
struct Puzzle {
    cells: [[Cell, ..9], ..9],
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
        for i in range(row_orig, row_orig + 3) {
            for j in range(col_orig, col_orig + 3) {
                self.cells[i][j].possibilities.remove(&val);
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
                        None    => fail!(),
                    };
                    self.set_item(col, row, val);
                    found_something = true;
                }
            }
        }
        println!("{}:{}:solve_select_single_possibility: found_something={}", file!(), line!(), found_something);
        found_something
    }

    fn solve_select_single_possible_location(&mut self) -> bool {
        let mut found_something = false;
        for row in range(0, self.cells.len()) {
            let mut val_counts = [0u, ..9];
            for (col, cell) in self.cells[row].iter().enumerate() {
                for val in cell.possibilities.iter() {
                    val_counts[val-1] += 1;
                }
            }
            for (col, cell) in self.cells[row].iter().enumerate() {
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
            let mut val_counts = [0u, ..9];
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
                let mut val_counts = [0u, ..9];

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

        println!("{}:{}:solve_select_single_possible_location: found_something={}", file!(), line!(), found_something);

        found_something
    }

    fn guess(&mut self) -> bool {
        if self.is_invalid() {
            return false;
        }

        // Ensure we try to guess first in cells with the fewest possibilities (i.e. biggest chance
        // of success)
        // TODO: persist this list in Puzzle and maintain it from set_item() using sorted inserts
        let mut to_search_cells : Vec<(uint, uint)> = Vec::from_fn(81, |i| std::num::div_rem(i, 9u));
        to_search_cells.retain(|&(row,col)| self.cells[row][col].possibilities.len() > 1);
        to_search_cells.sort_by(|&(row_a,col_a),&(row_b,col_b)| {
            let a = (self.cells[row_a][col_a].possibilities.len(), row_a, col_a);
            let b = (self.cells[row_b][col_b].possibilities.len(), row_b, col_b);
            a.cmp(&b)
        });

        for &(row_num, col_num) in to_search_cells.iter() {
            for possibility in self.cells[row_num][col_num].possibilities.iter() {
                let mut tmp = self.clone();
                tmp.set_item(col_num, row_num, possibility);

                println!("{}:{}:backtrack({}, {}, {} in {})", file!(), line!(),
                    col_num, row_num, possibility, self.cells[row_num][col_num].possibilities);

                if tmp.solve() {
                    self.clone_from(&tmp);
                    return true;
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

#[cfg(not(test))]
fn fmt_cell(cell : &Cell) -> &'static str {
    return match cell.value {
        Some(1) => "1",
        Some(2) => "2",
        Some(3) => "3",
        Some(4) => "4",
        Some(5) => "5",
        Some(6) => "6",
        Some(7) => "7",
        Some(8) => "8",
        Some(9) => "9",
        None    => " ",
        Some(n) => fail!("unexpected value {}", n),
    }
}

#[cfg(not(test))]
fn fmt_puzzle(puzzle : &Puzzle) {
    for row_idx in range(0, puzzle.cells.len() + 1) {
        match (row_idx / 3, row_idx % 3) {
            (0,0) => println!("┏━┯━┯━┳━┯━┯━┳━┯━┯━┓ ┏━┯━┯━┳━┯━┯━┳━┯━┯━┓"),
            (3,0) => println!("┗━┷━┷━┻━┷━┷━┻━┷━┷━┛ ┗━┷━┷━┻━┷━┷━┻━┷━┷━┛"),
            (_,0) => println!("┣━┿━┿━╋━┿━┿━╋━┿━┿━┫ ┣━┿━┿━╋━┿━┿━╋━┿━┿━┫"),
            (_,_) => println!("┠─┼─┼─╂─┼─┼─╂─┼─┼─┨ ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨"),
        };
        if row_idx >= puzzle.cells.len() {
            break
        }
        let row = &puzzle.cells[row_idx];
        println!("┃{}│{}│{}┃{}│{}│{}┃{}│{}│{}┃ ┃{}│{}│{}┃{}│{}│{}┃{}│{}│{}┃",
                 fmt_cell(&row[0]),
                 fmt_cell(&row[1]),
                 fmt_cell(&row[2]),
                 fmt_cell(&row[3]),
                 fmt_cell(&row[4]),
                 fmt_cell(&row[5]),
                 fmt_cell(&row[6]),
                 fmt_cell(&row[7]),
                 fmt_cell(&row[8]),
                 match row[0].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[1].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[2].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[3].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[4].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[5].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[6].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[7].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
                 match row[8].possibilities.len() { 0 => " ".to_owned(), n => format!("{}", n), },
            );
    }
}

/*
 * ┏━┯━┯━┳━┯━┯━┳━┯━┯━┓
 * ┃ │5│ ┃4│ │ ┃ │8│ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃4│ │6┃ │ │ ┃ │ │ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃ │ │3┃ │7│8┃ │ │ ┃
 * ┣━┿━┿━╋━┿━┿━╋━┿━┿━┫
 * ┃1│ │ ┃ │5│ ┃6│ │ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃ │ │8┃ │ │3┃ │4│ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃ │ │ ┃9│1│ ┃ │ │ ┃
 * ┣━┿━┿━╋━┿━┿━╋━┿━┿━┫
 * ┃2│ │9┃ │ │1┃ │ │ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃ │ │ ┃ │6│ ┃2│7│ ┃
 * ┠─┼─┼─╂─┼─┼─╂─┼─┼─┨
 * ┃ │7│ ┃ │ │ ┃8│6│ ┃
 * ┗━┷━┷━┻━┷━┷━┻━┷━┷━┛
 */
static a_puzzle : &'static str = concat!(
    ".5.4...8.",
    "4.6......",
    "..3.78...",
    "1...5.64.",
    "..8..3...",
    "...91....",
    "2.9..1...",
    "....6.27.",
    ".7....86.",
);

fn create_puzzle(inp : &str) -> Puzzle {
    let mut cur_puzzle : Puzzle = Default::default();
    for (i, c) in inp.chars().enumerate() {
        if i > 81 {
            break;
        }

        let (row_num, col_num) = std::num::div_rem(i, 9u);
        if '1' <= c && c <= '9' {
            cur_puzzle.set_item(col_num, row_num, (c as uint - '0' as uint));
        }
    }
    cur_puzzle
}

#[test]
fn can_solve_puzzle() {
    let mut cur_puzzle = create_puzzle(a_puzzle);
    assert!(cur_puzzle.solve());
    assert!(cur_puzzle.is_solved());
    assert!(!cur_puzzle.is_invalid());
}

#[cfg(not(test))]
fn main() {
    let mut cur_puzzle = create_puzzle(a_puzzle);
    fmt_puzzle(&cur_puzzle);
    cur_puzzle.solve();
    fmt_puzzle(&cur_puzzle);
}
