// https://adventofcode.com/2024/day/7

use anyhow::Result;
use std::env;
use utility::{get_raw_text, get_vec_from_txt};

fn eval_expr(nums: &[i64], ops: &[&str]) -> i64 {
    let mut result = nums[0];
    for (i, &num) in nums.iter().enumerate().skip(1) {
        match ops[i - 1] {
            "+" => result += num,
            "*" => result *= num,
            "||" => result = format!("{}{}", result, num).parse().unwrap(),
            op => unreachable!("There should be no such expression '{op}'"),
        }
    }
    result
}

fn _generate_ops<'a>(operators: &'a [&str], length: usize) -> Vec<Vec<&'a str>> {
    if length == 0 {
        return vec![];
    }

    let mut result = Vec::new();
    for op in operators {
        for mut smaller in generate_ops(operators, length - 1) {
            smaller.push(op);
            result.push(smaller);
        }
    }
    result
}

/// Based on python itertools.product
fn generate_ops<'a>(operators: &'a [&str], length: usize) -> Vec<Vec<&'a str>> {
    if length == 0 {
        return vec![];
    }

    let mut result = vec![vec![]];
    for _ in 0..length {
        let mut new_result = Vec::new();
        for combination in result.iter() {
            for &op in operators {
                let mut new_combination = combination.clone();
                new_combination.push(op);
                new_result.push(new_combination);
            }
        }
        result = new_result;
    }
    result
}

fn main() -> Result<()> {
    let ops1 = vec!["+", "*"];
    let ops2: Vec<&str> = ops1.iter().chain(&["||"]).cloned().collect();
    let filename = env::args().nth(1).unwrap_or("input.in".to_string());

    let txt = get_raw_text(&filename)?;

    let mut res = 0;
    let mut res2 = 0;

    for line in get_vec_from_txt(&txt) {
        let parts: Vec<&str> = line.split(": ").collect();
        let (lhs, rhs) = (parts[0], parts[1]);
        let expected: i64 = lhs.parse()?;
        let nums: Vec<i64> = rhs.split_whitespace().map(|s| s.parse().unwrap()).collect();

        for ops in generate_ops(&ops1, nums.len() - 1) {
            if eval_expr(&nums, &ops) == expected {
                res += expected;
                break;
            }
        }

        for ops in generate_ops(&ops2, nums.len() - 1) {
            if eval_expr(&nums, &ops) == expected {
                res2 += expected;
                break;
            }
        }
    }

    println!("{}", res);
    println!("{}", res2);

    Ok(())
}
