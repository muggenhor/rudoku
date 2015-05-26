/* vim: set et sts=4 sw=4: */

use rudoku::Puzzle;

impl Clone for Puzzle {
    fn clone(&self) -> Puzzle {
        Puzzle { cells: [
            [self.cells[0][0].clone(), self.cells[0][1].clone(), self.cells[0][2].clone(),
             self.cells[0][3].clone(), self.cells[0][4].clone(), self.cells[0][5].clone(),
             self.cells[0][6].clone(), self.cells[0][7].clone(), self.cells[0][8].clone(),],
            [self.cells[1][0].clone(), self.cells[1][1].clone(), self.cells[1][2].clone(),
             self.cells[1][3].clone(), self.cells[1][4].clone(), self.cells[1][5].clone(),
             self.cells[1][6].clone(), self.cells[1][7].clone(), self.cells[1][8].clone(),],
            [self.cells[2][0].clone(), self.cells[2][1].clone(), self.cells[2][2].clone(),
             self.cells[2][3].clone(), self.cells[2][4].clone(), self.cells[2][5].clone(),
             self.cells[2][6].clone(), self.cells[2][7].clone(), self.cells[2][8].clone(),],
            [self.cells[3][0].clone(), self.cells[3][1].clone(), self.cells[3][2].clone(),
             self.cells[3][3].clone(), self.cells[3][4].clone(), self.cells[3][5].clone(),
             self.cells[3][6].clone(), self.cells[3][7].clone(), self.cells[3][8].clone(),],
            [self.cells[4][0].clone(), self.cells[4][1].clone(), self.cells[4][2].clone(),
             self.cells[4][3].clone(), self.cells[4][4].clone(), self.cells[4][5].clone(),
             self.cells[4][6].clone(), self.cells[4][7].clone(), self.cells[4][8].clone(),],
            [self.cells[5][0].clone(), self.cells[5][1].clone(), self.cells[5][2].clone(),
             self.cells[5][3].clone(), self.cells[5][4].clone(), self.cells[5][5].clone(),
             self.cells[5][6].clone(), self.cells[5][7].clone(), self.cells[5][8].clone(),],
            [self.cells[6][0].clone(), self.cells[6][1].clone(), self.cells[6][2].clone(),
             self.cells[6][3].clone(), self.cells[6][4].clone(), self.cells[6][5].clone(),
             self.cells[6][6].clone(), self.cells[6][7].clone(), self.cells[6][8].clone(),],
            [self.cells[7][0].clone(), self.cells[7][1].clone(), self.cells[7][2].clone(),
             self.cells[7][3].clone(), self.cells[7][4].clone(), self.cells[7][5].clone(),
             self.cells[7][6].clone(), self.cells[7][7].clone(), self.cells[7][8].clone(),],
            [self.cells[8][0].clone(), self.cells[8][1].clone(), self.cells[8][2].clone(),
             self.cells[8][3].clone(), self.cells[8][4].clone(), self.cells[8][5].clone(),
             self.cells[8][6].clone(), self.cells[8][7].clone(), self.cells[8][8].clone(),],
         ],
         recursion_depth: self.recursion_depth,
        }
    }

    fn clone_from(&mut self, source: &Puzzle) {
        self.recursion_depth.clone_from(&source.recursion_depth);
        for (src_col, dst_col) in source.cells.iter().zip(self.cells.iter_mut()) {
            for (src_cell, dst_cell) in src_col.iter().zip(dst_col.iter_mut()) {
                dst_cell.clone_from(src_cell);
            }
        }
    }
}
