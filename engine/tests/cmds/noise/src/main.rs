use std::{thread, time};
use test_utils::{generate_random_string, get_arguments};

const DEFAULT_TIMEOUT_MS: u64 = 1000;
const DEFAULT_GENERATE_STDOUT: bool = true;
const DEFAULT_STR_LEN: usize = 100;
const DEFAULT_ITERATION_MS: u64 = 10;

fn main() {
    let args = get_arguments();
    let timeout: u64 = args
        .first()
        .map(|s| s.parse::<u64>().unwrap_or(DEFAULT_TIMEOUT_MS))
        .unwrap_or(DEFAULT_TIMEOUT_MS);
    let iteration: u64 = args
        .get(1)
        .map(|s| s.parse::<u64>().unwrap_or(DEFAULT_ITERATION_MS))
        .unwrap_or(DEFAULT_ITERATION_MS);
    let stdout = args
        .get(2)
        .map(|s| s.parse::<bool>().unwrap_or(DEFAULT_GENERATE_STDOUT))
        .unwrap_or(DEFAULT_GENERATE_STDOUT);
    let len: usize = args
        .get(3)
        .map(|s| s.parse::<usize>().unwrap_or(DEFAULT_STR_LEN))
        .unwrap_or(DEFAULT_STR_LEN);
    let started = time::Instant::now();
    if stdout {
        loop {
            println!("{}", generate_random_string(len));
            thread::sleep(time::Duration::from_millis(iteration));
            if started.elapsed().as_millis() >= timeout as u128 {
                break;
            }
        }
    } else {
        thread::sleep(time::Duration::from_millis(timeout));
    }
}
