/* vim: set et sts=4 sw=4: */

use rudoku::{ Cell, Puzzle };
use std::default::Default;

impl Default for Cell {
    fn default() -> Cell {
        Cell::new(&Default::default())
    }
}

impl Default for Puzzle {
    fn default() -> Puzzle {
        Puzzle { cells: [
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
            [Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),
             Default::default(), Default::default(), Default::default(),],
         ],
         recursion_depth: Default::default(),
        }
    }
}
