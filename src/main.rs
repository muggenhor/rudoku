/* vim: set et sts=4 sw=4: */

#[macro_use] extern crate log;
extern crate env_logger;

#[cfg(not(test))]
use rudoku::create_puzzle;

mod rudoku;
#[cfg(test)]
mod test;

#[cfg(not(test))]
fn main() {
    env_logger::init().unwrap();

    for e_line in std::io::stdin().lock().lines() {
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
