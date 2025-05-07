use std::io::Result;
use utility::*;

const MAX_DIST: u64 = 3;
const MIN_DIST: u64 = 1;

fn is_monotonic(nums: &[i64]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..nums.len() - 1 {
        let before = &nums[i];
        let after = &nums[i + 1];
        if after > before {
            decreasing = false;
        }
        if after < before {
            increasing = false;
        }
        if !increasing && !decreasing {
            break;
        }
    }

    increasing || decreasing
}

fn is_safe(nums: &[i64]) -> bool {
    if !is_monotonic(nums) {
        return false;
    }

    for i in 0..nums.len() - 1 {
        let diff = nums[i].abs_diff(nums[i + 1]);

        if !(MIN_DIST..=MAX_DIST).contains(&diff) {
            return false;
        }
    }

    true
}

fn is_safe_dampened(nums: &[i64]) -> bool {
    if is_safe(nums) {
        return true;
    }

    for i in 0..nums.len() {
        let mut other = nums.to_vec();
        other.remove(i);

        if is_safe(&other) {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    let txt = get_raw_text("input.txt")?;

    let mut safes = 0;
    let mut safes2 = 0;
    for line in get_vec_from_txt(&txt) {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if is_safe(&nums) {
            safes += 1;
        }
        if is_safe_dampened(&nums) {
            safes2 += 1;
        }
    }

    println!("Safes: {}", safes);
    println!("Safes, dampened: {}", safes2);

    Ok(())
}
