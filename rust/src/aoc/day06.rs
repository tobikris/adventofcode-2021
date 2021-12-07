use crate::aoc::input;

use std::collections::HashMap;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let mut fish = input::line_as_usize(day, 1).unwrap();
    format!(
        "Count of lanternfish after {} days: {}",
        80,
        simulate(&mut fish, 80)
    )
}

pub fn challenge2(day: usize) -> String {
    let mut fish = input::line_as_usize(day, 1).unwrap();
    format!(
        "Count of lanternfish after {} days: {}",
        256,
        simulate(&mut fish, 256)
    )
}

pub fn simulate(fish: &mut Vec<usize>, days: usize) -> usize {
    let mut fish_map = HashMap::<usize, usize>::new();
    for f in fish {
        *fish_map.entry(*f).or_insert(0) += 1;
    }
    for _ in 0..days {
        let mut new_fish_map = HashMap::<usize, usize>::new();
        for (state, count) in fish_map.iter() {
            if *state == 0 {
                *new_fish_map.entry(8).or_insert(0) += *count;
                *new_fish_map.entry(6).or_insert(0) += *count;
            } else {
                *new_fish_map.entry(state - 1).or_insert(0) += *count;
            }
        }
        fish_map = new_fish_map;
    }
    fish_map.iter().map(|(_k, v)| *v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let fish = vec![3, 4, 3, 1, 2];
        assert_eq!(simulate(&mut fish.clone(), 18), 26);
        assert_eq!(simulate(&mut fish.clone(), 80), 5934);
    }

    #[test]
    fn challenge2() {
        let mut fish = vec![3, 4, 3, 1, 2];
        assert_eq!(simulate(&mut fish, 256), 26984457539);
    }

    #[test]
    fn simulate_works() {
        let fish = vec![1];
        assert_eq!(simulate(&mut fish.clone(), 1), 1);
        assert_eq!(simulate(&mut fish.clone(), 2), 2);
        assert_eq!(simulate(&mut fish.clone(), 3), 2);
        assert_eq!(simulate(&mut fish.clone(), 4), 2);
        assert_eq!(simulate(&mut fish.clone(), 5), 2);
        assert_eq!(simulate(&mut fish.clone(), 6), 2);
        assert_eq!(simulate(&mut fish.clone(), 7), 2);
        assert_eq!(simulate(&mut fish.clone(), 8), 2);
        assert_eq!(simulate(&mut fish.clone(), 9), 3);
        assert_eq!(simulate(&mut fish.clone(), 10), 3);
        assert_eq!(simulate(&mut fish.clone(), 11), 4);
    }
}
