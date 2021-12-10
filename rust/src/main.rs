mod aoc;

use aoc::*;

fn main() {
    let days = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
    ];
    for (i, day) in days.iter().enumerate() {
        println!("day{:02}:", i + 1);
        day(i + 1);
    }
}
