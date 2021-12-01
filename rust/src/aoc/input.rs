use std::fs;

pub fn read_file(day: usize, challenge: usize) -> String {
    let filename = format!("../inputs/day{:02}-{}.txt", day, challenge);
    fs::read_to_string(filename).expect("error reading input")
}

pub fn as_ints(day: usize, challenge: usize) -> Result<Vec<i64>, std::num::ParseIntError> {
    read_file(day, challenge)
        .lines()
        .map(|integer| integer.parse())
        .collect()
}
