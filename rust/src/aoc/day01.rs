use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let integers = input::as_ints(day, 1);
    if let Ok(integers) = integers {
        let increases = count_depth_increases_win(&integers, 2);
        return format!("Increasing depth: {} times", increases);
    }
    "".to_string()
}

pub fn challenge2(day: usize) -> String {
    let integers = input::as_ints(day, 1);
    if let Ok(integers) = integers {
        let increases = count_depth_increases_win(&integers, 4);
        return format!("Increasing depth in windows of 3: {} times", increases);
    }
    "".to_string()
}

fn count_depth_increases_win(measurements: &Vec<i64>, window: usize) -> usize {
    measurements
        .windows(window)
        .filter_map(|w| match w.first().copied() < w.last().copied() {
            true => Some(true),
            false => None,
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let numbers = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = count_depth_increases_win(&numbers, 2);
        assert_eq!(count, 7)
    }

    #[test]
    fn challenge2() {
        let numbers = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let count = count_depth_increases_win(&numbers, 4);
        assert_eq!(count, 5);
    }
}
