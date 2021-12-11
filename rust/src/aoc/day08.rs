use crate::aoc::input;

use bimap::BiMap;

use itertools::Itertools;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let outputs = read
        .lines()
        .map(|l| {
            l.split(" | ")
                .last()
                .unwrap()
                .split(" ")
                .collect::<Vec<_>>()
        })
        .collect();
    let count = count_unique_combinations(outputs);
    format!("The digits 1, 4, 7, or 8 appear {} times", count)
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let sum: usize = read
        .lines()
        .map(|l| {
            let l = l
                .split(" | ")
                .map(|v| v.split(" ").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            let digits = &mut decode_output(l[0].clone(), l[1].clone());
            let mut number = 0;
            digits.reverse();
            let base: usize = 10;
            for i in 0..digits.len() {
                number += digits[i] * (base.pow(i as u32))
            }
            number
        })
        .sum();
    format!("The sum of all decoded numbers is {}", sum)
}

pub fn count_unique_combinations(outputs: Vec<Vec<&str>>) -> usize {
    outputs
        .iter()
        .map(|l| {
            l.iter()
                .filter_map(|o| match is_unique_combination(o) {
                    true => Some(o),
                    _ => None,
                })
                .count()
        })
        .sum()
}

pub fn is_unique_combination(output: &str) -> bool {
    match output.len() {
        2 => true,
        3 => true,
        4 => true,
        7 => true,
        _ => false,
    }
}

pub fn decode_output(input: Vec<&str>, output: Vec<&str>) -> Vec<usize> {
    let mut lookup = BiMap::new();

    let mut iteration = 0;
    while lookup.len() != 10 && iteration < 25 {
        iteration += 1;
        for i in input.iter() {
            let isorted = i.chars().sorted().collect::<String>();
            if lookup.get_by_left(&isorted).is_some() {
                continue;
            }
            match i.len() {
                2 => {
                    lookup.insert(isorted, 1);
                }
                3 => {
                    lookup.insert(isorted, 7);
                }
                4 => {
                    lookup.insert(isorted, 4);
                }
                7 => {
                    lookup.insert(isorted, 8);
                }
                6 => {
                    let segments = lookup.get_by_right(&4);
                    if segments.is_some()
                        && segments
                            .unwrap()
                            .chars()
                            .filter(|&s| isorted.contains(s))
                            .count()
                            == segments.unwrap().len()
                    {
                        lookup.insert(isorted, 9);
                        continue;
                    }
                    let segments = lookup.get_by_right(&1);
                    if segments.is_some()
                        && segments
                            .unwrap()
                            .chars()
                            .filter(|&s| isorted.contains(s))
                            .count()
                            == segments.unwrap().len()
                    {
                        lookup.insert(isorted, 0);
                        continue;
                    }

                    lookup.insert(isorted, 6);
                }
                5 => {
                    let segments = lookup.get_by_right(&6);
                    if segments.is_some()
                        && isorted
                            .chars()
                            .filter(|&s| segments.unwrap().contains(s))
                            .count()
                            == isorted.len()
                    {
                        lookup.insert(isorted, 5);
                        continue;
                    }
                    let segments = lookup.get_by_right(&9);
                    if segments.is_some()
                        && isorted
                            .chars()
                            .filter(|&s| segments.unwrap().contains(s))
                            .count()
                            == isorted.len()
                    {
                        lookup.insert(isorted, 3);
                        continue;
                    }

                    lookup.insert(isorted, 2);
                }
                _ => {}
            };
        }
    }
    output
        .iter()
        .map(|l| {
            *lookup
                .get_by_left(&l.chars().sorted().collect::<String>())
                .unwrap_or(&10)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(8, 0);
        let outputs = read
            .lines()
            .map(|l| {
                l.split(" | ")
                    .last()
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<_>>()
            })
            .collect();
        let count = count_unique_combinations(outputs);
        assert_eq!(count, 26);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(8, 0);
        let sum: usize = read
            .lines()
            .map(|l| {
                let l = l
                    .split(" | ")
                    .map(|v| v.split(" ").collect::<Vec<&str>>())
                    .collect::<Vec<Vec<&str>>>();
                let digits = &mut decode_output(l[0].clone(), l[1].clone());
                let mut number = 0;
                digits.reverse();
                let base: usize = 10;
                for i in 0..digits.len() {
                    number += digits[i] * (base.pow(i as u32))
                }
                number
            })
            .sum();
        assert_eq!(sum, 61229);
    }

    #[test]
    fn count_unique_combinations_works() {
        let outputs = vec!["ab", "abc", "abcd", "abcde", "abcdef", "abcdefg"];
        let count = outputs
            .iter()
            .filter_map(|o| match is_unique_combination(o) {
                true => Some(o),
                _ => None,
            })
            .count();
        assert_eq!(count, 4);
    }

    #[test]
    fn decode_output_works() {
        let read =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let read = read
            .split(" | ")
            .map(|v| v.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let output = decode_output(read[0].clone(), read[1].clone());
        assert_eq!(output, vec![5, 3, 5, 3]);
    }
}
