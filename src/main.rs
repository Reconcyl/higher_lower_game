extern crate rand;
use rand::distributions::{Sample, Normal};
use rand::{random, thread_rng};

const TRIALS: usize = 100_000_000;

fn pick_float() -> f64 {
    Normal::new(0.0, 10000.0).sample(&mut thread_rng())
}

fn random_strategy(_number_seen: f64) -> bool {
    random()
}

fn good_strategy(number_seen: f64) -> bool {
    number_seen > pick_float()
}

fn run_trial_with<S: Fn(f64) -> bool>(strategy: &S) -> bool {
    let first_number = pick_float();
    strategy(first_number) == (first_number > pick_float())
}

fn run_trials_with<S: Fn(f64) -> bool>(strategy: &S, trials: usize) -> usize {
    (0..trials).filter(|_| run_trial_with(strategy)).count()
}

fn main() {
    println!("Starting...");
    println!("Random strategy wins {} out of {} times",
             run_trials_with(&random_strategy, TRIALS), TRIALS);
    println!("Good strategy wins {} out of {} times",
             run_trials_with(&good_strategy, TRIALS), TRIALS);
}
