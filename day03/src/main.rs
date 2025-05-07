use regex_lite::Regex;
use utility::get_raw_text;

fn main() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let hay = get_raw_text("input.txt").unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(&hay) {
        let a: i32 = cap[1].parse().expect("All elements should be parsable");
        let b: i32 = cap[2].parse().expect("All elements should be parsable");

        sum += a * b;
    }

    println!("Sum : {}", sum);
}
