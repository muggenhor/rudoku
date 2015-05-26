/* vim: set et sts=4 sw=4: */

#![feature(collections)]

#[macro_use] extern crate log;
extern crate env_logger;

#[cfg(not(test))]
use rudoku::create_puzzle;
#[cfg(not(test))]
use std::io::BufRead;

mod rudoku;
#[cfg(test)]
mod test;

#[cfg(not(test))]
fn main() {
    env_logger::init().unwrap();

    let stdin = &std::io::stdin();
    for e_line in stdin.lock().lines() {
        match e_line {
            Ok(line) => {
                let mut cur_puzzle = create_puzzle(&line);
                println!("{}", cur_puzzle);
                cur_puzzle.solve();
                println!("{}", cur_puzzle);
            },
            Err(e) => error!("error reading: {}", e),
        }
    }
}
