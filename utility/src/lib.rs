use std::any::type_name;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Result};
use std::str::FromStr;

pub fn get_raw_text(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_vec_from_txt(txt: &str) -> Vec<String> {
    txt.lines().map(String::from).collect()
}

pub fn parse_str_vec<T>(vec: &[String]) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut new: Vec<T> = Vec::new();
    for (i, s) in vec.iter().enumerate() {
        let parsed = match T::from_str(s) {
            Ok(val) => val,
            Err(_) => {
                panic!(
                    "Failed to parse element {}, `{}` into {}.",
                    i,
                    s,
                    type_name::<T>()
                );
            }
        };

        new.push(parsed);
    }
    new
}
