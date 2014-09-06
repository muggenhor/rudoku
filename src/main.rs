/* vim: set et sts=4 sw=4: */

#![feature(phase)]
#![reexport_test_harness_main = "test_main"]
#[phase(plugin)] extern crate green;
#[phase(plugin, link)] extern crate log;

extern crate green;
extern crate rustuv;

#[cfg(not(test))]
use rudoku::create_puzzle;

mod rudoku;
#[cfg(test)]
mod test;

#[cfg(test)] #[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, test_main)
}
#[cfg(not(test))]
green_start!(main)

#[cfg(not(test))]
fn main() {
    for e_line in std::io::stdin().lines() {
        match e_line {
            Ok(line) => {
                let mut cur_puzzle = create_puzzle(line.as_slice());
                println!("{}", cur_puzzle);
                cur_puzzle.solve();
                println!("{}", cur_puzzle);
            },
            Err(e) => error!("error reading: {}", e),
        }
    }
}
