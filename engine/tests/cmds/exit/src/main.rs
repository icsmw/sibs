use std::{process::exit, thread, time};
use test_utils::{generate_random_string, get_arguments};

const DEFAULT_EXIT_CODE: i32 = 0;
const DEFAULT_TIMEOUT_MS: u64 = 10;
const DEFAULT_STR_LEN: usize = 100;
const DEFAULT_ITERATION_MS: u64 = 10;

/// Program for testing spawning in the scope of SIBS
/// exit <exit_code {number}> <sleep_time {number}> <iteration_time {number}> <len {number}>
///     exit_code       - code to be used for exit
///     sleep_time      - duration of program work
///     iteration_time  - interval to post random content into stdout
///     len             - length of each string posted into stdout
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
