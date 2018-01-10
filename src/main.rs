extern crate rand;
extern crate gcd;

use rand::Rng;
use gcd::Gcd;
use std::f64::consts::PI;

fn main() {
    let max_random: u64 = 256u64.pow(3);
    let max_iterations: u64 = 256u64.pow(2);
    
    let mut count: u64 = 0;
    let mut lowest_error: f64 = PI;
    let mut best_pi: f64 = 0.0;

    for i in 1..max_iterations {
        let (a, b) = random_pair(max_random);

        if Gcd::gcd(a, b) == 1 {
            count += 1;
        }
        println!("C: {}, I: {}", count, i);

        if count > 0 {
            let pi: f64 = pi_from_probability(count, i);
            let error: f64 = (pi - PI).abs();
            
            println!("[{}/{}]:", i, max_iterations);
            println!("Result: {}", pi.to_string());
            println!("Error: {}", error.to_string());

            if error < lowest_error {
                lowest_error = error;
                best_pi = pi;
            }
        }
    }

    let accuracy: f64 = (lowest_error / PI) * 100.0;
    println!("
            \nComputed Pi: {}
            \n  Actual Pi: {}
            \n
            \nAccuracy: {}
            \nLowest Error: {} ", 
            best_pi, PI, accuracy, lowest_error);
}

fn random_pair(max_random: u64) -> (u64, u64) {
    let a: u64 = rand::thread_rng().gen_range(1, max_random);
    let b: u64 = rand::thread_rng().gen_range(1, max_random);
    return (a, b);
}

fn pi_from_probability(count: u64, iteration: u64) -> f64 {
    let c: f64 = count as f64 / iteration as f64;
    let pi: f64 = (6.0/c).sqrt();
    return pi;
}
