use std::collections::HashMap;
use utility::{get_raw_text, get_vec_from_txt};

fn main() {
    let txt = get_raw_text("input.txt").unwrap();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in get_vec_from_txt(&txt) {
        for (i, elem) in line.split_whitespace().enumerate() {
            let n: i64 = elem.parse().expect("All elements should be parsable");
            match i {
                0 => left.push(n),
                1 => right.push(n),
                _ => unreachable!("There should be only two values per line"),
            }
        }
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        let distance = left[i].abs_diff(right[i]);
        sum += distance;
    }

    println!("Part 1: {}", sum);

    // Part 2
    let mut appears_on_right = HashMap::new();

    for r in &right {
        *appears_on_right.entry(r).or_insert(0) += 1;
    }

    let sum2: i64 = left
        .iter()
        .filter_map(|l| appears_on_right.get(l).map(|&count| l * count))
        .sum();

    println!("Part 2: {}", sum2);
}
