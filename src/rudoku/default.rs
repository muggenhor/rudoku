/* vim: set et sts=4 sw=4: */

use rudoku::Cell;
use std::default::Default;

impl Default for Cell {
    fn default() -> Cell {
        Cell::new(&Default::default())
    }
}

impl Default for [Cell; 9] {
    fn default() -> [Cell; 9] {
        [
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
        ]
    }
}

impl Default for [[Cell; 9]; 9] {
    fn default() -> [[Cell; 9]; 9] {
        [
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
            Default::default(), Default::default(), Default::default(),
        ]
    }
}
