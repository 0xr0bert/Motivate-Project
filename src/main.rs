extern crate motivatelib;

use std::env;
use std::fs::File;

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
    // Load parameters from file
    let parameters = motivatelib::Parameters::from_file(
        File::open("config/parameters.yaml")
            .expect("Failed to open parameters file")
    );

    motivatelib::run_simulation(generate, parameters);
}