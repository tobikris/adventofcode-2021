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
    let read = input::read_file(day, 1);
    let lines = read.lines().collect();
    let (oxy, co2) = calc_life_support_rating(&lines);
    return format!("Oxygen {} * CO2 {} = {:}", oxy, co2, oxy * co2);
}

fn calc_power_consumption(measurements: &Vec<&str>) -> (String, String) {
    let transposed = transpose(measurements.iter().map(|l| l.chars().collect()).collect());
    let gamma = transposed
        .iter()
        .map(|c| {
            let ones = c.iter().filter(|ch| **ch == '1').count();
            match ones > c.len() / 2 {
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

fn reduce(vec: &Vec<&str>, matcher: fn(ones: usize, zeros: usize) -> char) -> i64 {
    let mut vec = vec.clone();
    let mut column: usize = 0;
    while vec.len() > 1 {
        let comp = transpose(vec.iter().map(|l| l.chars().collect()).collect())
            .iter()
            .map(|c| {
                let ones = c.iter().filter(|ch| **ch == '1').count();
                let zeros = c.iter().filter(|ch| **ch == '0').count();
                matcher(ones, zeros)
            })
            .collect::<Vec<_>>()[column];
        vec.retain(|&m| m.chars().collect::<Vec<_>>()[column] == comp);
        column += 1;
    }
    isize::from_str_radix(vec.iter().collect::<Vec<_>>()[0], 2)
        .unwrap()
        .try_into()
        .unwrap()
}

fn calc_life_support_rating(measurements: &Vec<&str>) -> (i64, i64) {
    let oxy = reduce(&measurements.clone(), |ones, zeros: usize| {
        match ones >= zeros {
            true => '1',
            _ => '0',
        }
    });
    let co2 = reduce(&measurements.clone(), |ones, zeros: usize| {
        match ones < zeros {
            true => '1',
            _ => '0',
        }
    });
    (oxy, co2)
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

    #[test]
    fn challenge2() {
        let steps = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let (oxygen, co2) = calc_life_support_rating(&steps);
        assert_eq!(oxygen, 23);
        assert_eq!(co2, 10);
    }
}
