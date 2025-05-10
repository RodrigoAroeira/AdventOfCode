use regex_lite::Regex;
use utility::get_raw_text;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let hay = get_raw_text("input.txt").unwrap();

    let mut sum = 0;
    let mut sum2 = 0;

    let mut should_mul = true;

    for cap in re.captures_iter(&hay) {
        let cap_str = &cap[0];

        if cap_str == "do()" {
            should_mul = true;
            continue;
        } else if cap_str == "don't()" {
            should_mul = false;
            continue;
        };

        let a: i32 = cap[1].parse().expect("All elements should be parsable");
        let b: i32 = cap[2].parse().expect("All elements should be parsable");

        sum += a * b;

        if should_mul {
            sum2 += a * b;
        }
    }

    println!("Sum : {sum}");
    println!("Sum2: {sum2}")
}
