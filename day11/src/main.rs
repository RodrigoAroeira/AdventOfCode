use std::{collections::HashMap, io, str::FromStr};

use utility::get_raw_text;

type IntType = u128;

fn num_len(n: IntType) -> usize {
    match n {
        0 => 1,
        _ => n.ilog10() as usize + 1,
    }
}

fn string_to_int(s: &str) -> IntType {
    IntType::from_str(s).unwrap()
}

fn even_digits_rule(n: IntType) -> [IntType; 2] {
    let as_str = n.to_string();
    let division_point = as_str.len() / 2;

    let half1 = string_to_int(&as_str[..division_point]);
    let half2 = string_to_int(&as_str[division_point..]);

    [half1, half2]
}

fn else_rule(n: IntType) -> IntType {
    n * 2024
}

fn count_cached(
    stone: IntType,
    steps: IntType,
    cache: &mut HashMap<(IntType, IntType), IntType>,
) -> IntType {
    let key = (stone, steps);

    if let Some(&cached) = cache.get(&key) {
        return cached;
    }

    if steps == 0 {
        return 1;
    }

    if stone == 0 {
        return count_cached(1, steps - 1, cache);
    }

    let res = if num_len(stone) % 2 == 0 {
        let [half1, half2] = even_digits_rule(stone);
        count_cached(half1, steps - 1, cache) + count_cached(half2, steps - 1, cache)
    } else {
        count_cached(else_rule(stone), steps - 1, cache)
    };

    cache.insert(key, res);
    res
}

fn main() -> io::Result<()> {
    let txt = get_raw_text("input.txt")?;
    let mut stones = Vec::new();

    for word in txt.split_whitespace() {
        stones.push(string_to_int(word));
    }

    let mut cache = HashMap::new();
    for blink in [25, 75] {
        let mut sum = 0;
        for &stone in &stones {
            sum += count_cached(stone, blink, &mut cache);
        }
        println!("{}", sum);
    }

    Ok(())
}
