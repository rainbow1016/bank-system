use std::env;
use std::process;

use crate::ledger::init_bank;

mod bank;
mod ledger;

// cargo run -- 3 ledger.txt
// cargo run -- 3 pressure_test.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <num_of_threads> <leader_file>", args[0]);
        process::exit(-1);
    }

    let p = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid number of threads specified");
            process::exit(-1);
        }
    };

    init_bank(p as usize, &args[2]);
    // println!("Hello, world!");
}
