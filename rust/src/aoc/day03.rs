use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let lines = read.lines().collect();
    let (gamma, epsilon) = calc_power_consumption(&lines);
    return format!(
        "Gamma {} * Epsilon {} = {:}",
        gamma,
        epsilon,
        isize::from_str_radix(gamma.as_str(), 2).unwrap()
            * isize::from_str_radix(epsilon.as_str(), 2).unwrap()
    );
}

pub fn challenge2(day: usize) -> String {
    "".to_string()
}

fn calc_power_consumption(measurements: &Vec<&str>) -> (String, String) {
    let transposed = transpose(measurements.iter().map(|l| l.chars().collect()).collect());
    let gamma = transposed
        .iter()
        .map(|c| {
            let zeros = c.iter().filter(|ch| **ch == '1').count();
            match zeros > c.len() / 2 {
                true => '1',
                false => '0',
            }
        })
        .collect::<String>();
    let epsilon = gamma
        .chars()
        .map(|c| match c == '1' {
            true => '0',
            false => '1',
        })
        .collect::<String>();
    (gamma, epsilon)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let steps = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let (gamma, epsilon) = calc_power_consumption(&steps);
        assert_eq!(gamma, "10110");
        assert_eq!(epsilon, "01001");
        assert_eq!(
            isize::from_str_radix(gamma.as_str(), 2).unwrap()
                * isize::from_str_radix(epsilon.as_str(), 2).unwrap(),
            198
        );
    }
}
