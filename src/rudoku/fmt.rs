/* vim: set et sts=4 sw=4: */

use rudoku::{ Cell, Puzzle };
use std::fmt;
use std::num;

impl fmt::Show for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(n) => {
                assert!(n >= 1 && n <= 9);
                write!(f, "{:u}", n)
            },
            None    => write!(f, " "),
        }
    }
}

impl fmt::Show for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_idx, row) in self.cells.iter().enumerate() {
            match match num::div_rem(row_idx, 3) {
                (0,0) => writeln!(f, "┏━┯━┯━┳━┯━┯━┳━┯━┯━┓"),
                (_,0) => writeln!(f, "┣━┿━┿━╋━┿━┿━╋━┿━┿━┫"),
                (_,_) => writeln!(f, "┠─┼─┼─╂─┼─┼─╂─┼─┼─┨"),
            } {
                Err(e) => return Err(e),
                Ok(_) => (),
            }
            match writeln!(f, "┃{}│{}│{}┃{}│{}│{}┃{}│{}│{}┃",
                     row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8],
            ) {
                Err(e) => return Err(e),
                Ok(_) => (),
            }
        }
        write!(f, "┗━┷━┷━┻━┷━┷━┻━┷━┷━┛")
    }
}
