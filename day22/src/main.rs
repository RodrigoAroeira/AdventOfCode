// https://adventofcode.com/2024/day/22

use std::io::Result;
use utility::{get_raw_text, get_vec_from_txt, parse_str_vec};

type NumType = i64;

const STEPS: NumType = 2000;
const PRUNE_MOD: NumType = 16777216;
const STEP1_MULT: f64 = 64.0;
const STEP2_MULT: f64 = 1.0 / 32.0;
const STEP3_MULT: f64 = 2048.0;

fn mix(secret: NumType, value: NumType) -> NumType {
    secret ^ value
}

fn prune(secret: NumType) -> NumType {
    secret % PRUNE_MOD
}

fn step(secret: NumType, value: f64) -> NumType {
    let mult = (secret as f64 * value) as NumType;
    let mixed = mix(secret, mult);

    prune(mixed)
}

fn full_loop(secret: NumType) -> NumType {
    let step1 = step(secret, STEP1_MULT);
    let step2 = step(step1, STEP2_MULT);
    step(step2, STEP3_MULT)
}

#[allow(dead_code)] // For part 2, which is incomplete
fn get_last_digit(n: NumType) -> NumType {
    n % 10
}

fn main() -> Result<()> {
    let txt = get_raw_text("input.in")?;
    let lines = get_vec_from_txt(txt.clone());
    let parsed: Vec<NumType> = parse_str_vec(&lines);

    let mut sum1 = 0;
    let mut _sum2 = 0;
    let mut last_digit = 0;

    for mut secret in parsed {
        let curr_digit = get_last_digit(secret);

        for _ in 0..STEPS {
            secret = full_loop(secret);
        }

        sum1 += secret;

        let diff = curr_digit - last_digit;
        _sum2 += diff;
        last_digit = curr_digit;
    }

    println!("Sum1: {}", sum1);
    // println!("Sum2: {}", _sum2);
    Ok(())
}
