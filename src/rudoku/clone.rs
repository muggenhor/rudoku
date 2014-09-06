/* vim: set et sts=4 sw=4: */

use rudoku::Cell;
use std::default::Default;

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
