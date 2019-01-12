extern crate motivatelib;

use std::env;

/// This is the entry point for the application
fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut generate = false;
    if args.len() >= 2 {
        if &args[1] == "--generate" {
            generate = true;
        }
    }
    motivatelib::run_simulation(generate);
}