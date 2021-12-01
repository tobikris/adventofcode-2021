use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let integers = input::as_ints(day, 1);
    if let Ok(integers) = integers {
        let increases = integers
            .windows(2)
            .map(|w| w[0] < w[1])
            .filter(|w| *w)
            .count();
        return format!("Increasing depth: {} times", increases);
    }
    "".to_string()
}

pub fn challenge2(day: usize) -> String {
    let integers = input::as_ints(day, 1);
    if let Ok(integers) = integers {
        let increases = integers
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| w[0] < w[1])
            .filter(|w| *w)
            .count();
        return format!("Increasing depth in windows of 3: {} times", increases);
    }
    "".to_string()
}
