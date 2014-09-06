/* vim: set et sts=4 sw=4: */

use rudoku::{ create_puzzle, Puzzle };
use std::default::Default;

#[test]
fn test_can_solve_puzzle() {
    let mut cur_puzzle = create_puzzle(concat!(
        ".5.4...8.",
        "4.6......",
        "..3.78...",
        "1...5.64.",
        "..8..3...",
        "...91....",
        "2.9..1...",
        "....6.27.",
        ".7....86.",
    ));
    assert!(cur_puzzle.solve());
    assert!(cur_puzzle.is_solved());
    assert!(!cur_puzzle.is_invalid());
}

#[test]
fn test_can_solve_empty_puzzle() {
    let mut cur_puzzle : Puzzle = Default::default();
    assert!(cur_puzzle.solve());
    assert!(cur_puzzle.is_solved());
    assert!(!cur_puzzle.is_invalid());
}
