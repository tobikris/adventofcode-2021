use crate::aoc::input;

use recap::Recap;
use serde::Deserialize;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let lines = read_lines(read);
    let mut field = Field::new(1000);
    lines
        .iter()
        .filter(|l| l.horiz_vert())
        .for_each(|l| field.add(l));
    return format!(
        "Count of fields crossed by 2 or more vent lines: {}",
        field.overlaps()
    );
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let lines = read_lines(read);
    let mut field = Field::new(1000);
    lines
        .iter()
        .filter(|l| l.horiz_vert() || l.diag())
        .for_each(|l| field.add(l));
    return format!(
        "Count of fields crossed by 2 or more vent lines: {}",
        field.overlaps()
    );
}

pub struct Field {
    fields: Vec<Vec<usize>>,
}

impl Field {
    pub fn new(size: usize) -> Field {
        Field {
            fields: vec![vec![0; size]; size],
        }
    }
    pub fn add(&mut self, line: &Line) {
        let mut x_range = if line.x1 < line.x2 {
            (line.x1..=line.x2).collect::<Vec<usize>>()
        } else {
            (line.x2..=line.x1).rev().collect::<Vec<usize>>()
        };
        let mut y_range = if line.y1 < line.y2 {
            (line.y1..=line.y2).collect::<Vec<usize>>()
        } else {
            (line.y2..=line.y1).rev().collect::<Vec<usize>>()
        };
        if x_range.len() < y_range.len() {
            x_range = x_range
                .iter()
                .cycle()
                .take(y_range.len())
                .map(|v| *v)
                .collect();
        }
        if y_range.len() < x_range.len() {
            y_range = y_range
                .iter()
                .cycle()
                .take(x_range.len())
                .map(|v| *v)
                .collect();
        }
        for (x, y) in x_range.iter().zip(y_range) {
            self.fields[*x][y] += 1;
        }
    }
    pub fn overlaps(&self) -> usize {
        self.fields
            .iter()
            .map(|x| x.iter().filter(|f| **f >= 2).count())
            .sum()
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for x in &self.fields {
            for y in x {
                write!(f, "{}", y).expect("");
            }
            write!(f, "\n").expect("");
        }
        write!(f, "\n")
    }
}

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")]
pub struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    pub fn horiz_vert(&self) -> bool {
        if self.x1 != self.x2 && self.y1 != self.y2 {
            return false;
        }
        true
    }
    pub fn diag(&self) -> bool {
        let ratio = ((self.x2 as f32 - self.x1 as f32) / (self.y2 as f32 - self.y1 as f32)).abs();
        if ratio == 1.0 {
            return true;
        }
        false
    }
}

pub fn read_lines(lines: String) -> Vec<Line> {
    lines.lines().filter_map(|l| l.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(5, 0);
        let lines = read_lines(read);
        assert_eq!(lines[0].x1, 0);
        assert_eq!(lines[0].y1, 9);
        let mut field = Field::new(10);
        lines
            .iter()
            .filter(|l| l.horiz_vert())
            .for_each(|l| field.add(l));
        assert_eq!(field.overlaps(), 5);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(5, 0);
        let lines = read_lines(read);
        assert_eq!(lines[0].x1, 0);
        assert_eq!(lines[0].y1, 9);
        let mut field = Field::new(10);
        lines
            .iter()
            .filter(|l| l.horiz_vert() || l.diag())
            .for_each(|l| field.add(l));
        assert_eq!(field.overlaps(), 12);
    }

    #[test]
    fn line_diag() {
        let lines = vec!["1,1 -> 3,3", "3,3 -> 1,1", "9,7 -> 7,9", "4,3 -> 6,5"];
        for l in lines {
            let line: Line = l.parse().unwrap();
            assert_eq!(line.diag(), true, "{}", l);
        }
    }
}
