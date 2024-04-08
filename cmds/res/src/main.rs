use common::{generate_random_string, get_arguments};
use std::{process::exit, thread, time};

const DEFAULT_EXIT_CODE: i32 = 0;
const DEFAULT_TIMEOUT_MS: u64 = 10;
const DEFAULT_STR_LEN: usize = 100;
const DEFAULT_ITERATION_MS: u64 = 10;

fn main() {
    let args = get_arguments();
    let exit_code: i32 = args
        .first()
        .map(|s| s.parse::<i32>().unwrap_or(DEFAULT_EXIT_CODE))
        .unwrap_or(DEFAULT_EXIT_CODE);
    let timeout: u64 = args
        .get(1)
        .map(|s| s.parse::<u64>().unwrap_or(DEFAULT_TIMEOUT_MS))
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    let iteration: u64 = args
        .get(2)
        .map(|s| s.parse::<u64>().unwrap_or(DEFAULT_ITERATION_MS))
        .unwrap_or(DEFAULT_ITERATION_MS);
    let len: usize = args
        .get(3)
        .map(|s| s.parse::<usize>().unwrap_or(DEFAULT_STR_LEN))
        .unwrap_or(DEFAULT_STR_LEN);
    let started = time::Instant::now();
    loop {
        println!("{}", generate_random_string(len));
        thread::sleep(time::Duration::from_millis(iteration));
        if started.elapsed().as_millis() >= timeout as u128 {
            break;
        }
    }
    if exit_code != 0 {
        eprintln!("{}", generate_random_string(len));
    }
    exit(exit_code)
}
