use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let numbers = read_drawn_numbers(read.lines().next().unwrap().to_string());
    let mut boards = read_boards(read.lines().skip(2), 5);
    let (score, number) = play(&mut boards, numbers);
    return format!("Score {} * Number {} = {:}", score, number, score * number);
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let numbers = read_drawn_numbers(read.lines().next().unwrap().to_string());
    let mut boards = read_boards(read.lines().skip(2), 5);
    let (score, number) = play_last(&mut boards, numbers);
    return format!("Score {} * Number {} = {:}", score, number, score * number);
}

pub fn read_drawn_numbers(file: String) -> Vec<usize> {
    file.split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<isize>,
    gridsize: usize,
}

impl Board {
    pub fn play(&mut self, number: isize) -> bool {
        let neg: isize = -1;
        self.grid.iter_mut().for_each(|e| {
            if *e == number {
                *e = neg
            }
        });
        self.has_won()
    }
    pub fn has_won(&self) -> bool {
        let neg: isize = -1;
        for index in 0..self.gridsize {
            if self.row(index).iter().sum::<isize>()
                <= TryInto::<isize>::try_into(self.gridsize).unwrap() * neg
            {
                return true;
            }
        }
        for index in 0..self.gridsize {
            if self.col(index).iter().sum::<isize>()
                <= TryInto::<isize>::try_into(self.gridsize).unwrap() * neg
            {
                return true;
            }
        }
        false
    }
    pub fn score(&self) -> usize {
        self.grid
            .iter()
            .filter(|v| **v != -1)
            .sum::<isize>()
            .try_into()
            .unwrap()
    }
    pub fn row(&self, index: usize) -> Vec<isize> {
        self.grid
            .windows(5)
            .nth(index * 5)
            .unwrap()
            .try_into()
            .unwrap()
    }
    pub fn col(&self, index: usize) -> Vec<isize> {
        self.grid
            .iter()
            .skip(index)
            .step_by(5)
            .map(|e| *e)
            .collect::<Vec<_>>()
    }
}

pub fn read_boards(lines: std::iter::Skip<std::str::Lines<'_>>, gridsize: usize) -> Vec<Board> {
    lines
        .fold(String::new(), |a, l| a + l + "\n")
        .split("\n\n")
        .map(|b| Board {
            gridsize: gridsize,
            grid: b
                .to_string()
                .split_ascii_whitespace()
                .map(|e| e.parse::<isize>().unwrap())
                .collect(),
        })
        .collect::<Vec<_>>()
}

pub fn play_boards(boards: &mut Vec<Board>, number: isize) -> Option<&Board> {
    let mut won: Option<&Board> = None;
    for b in boards.iter_mut() {
        if b.play(number) {
            won = Some(b);
        }
    }
    return won;
}

pub fn play(boards: &mut Vec<Board>, numbers: Vec<usize>) -> (usize, usize) {
    for n in numbers.iter() {
        if let Some(b) = play_boards(boards, TryInto::<isize>::try_into(*n).unwrap()) {
            return (b.score(), *n);
        }
    }
    (0, 0)
}

pub fn play_last(boards: &mut Vec<Board>, numbers: Vec<usize>) -> (usize, usize) {
    let mut score = 0;
    let mut number = 0;
    for n in numbers.iter() {
        if let Some(b) = play_boards(boards, TryInto::<isize>::try_into(*n).unwrap()) {
            score = b.score();
            number = *n;
            boards.retain(|b| !b.has_won());
        }
    }
    (score, number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(4, 0);
        let numbers = read_drawn_numbers(read.lines().next().unwrap().to_string());
        let mut boards = read_boards(read.lines().skip(2), 5);
        let (score, number) = play(&mut boards, numbers);
        assert_eq!(score, 188);
        assert_eq!(number, 24);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(4, 0);
        let numbers = read_drawn_numbers(read.lines().next().unwrap().to_string());
        let mut boards = read_boards(read.lines().skip(2), 5);
        println!("{:?}", boards);
        let (score, number) = play_last(&mut boards, numbers);
        assert_eq!(score, 148);
        assert_eq!(number, 13);
    }

    #[test]
    fn board_row() {
        let read = input::read_file(4, 0);
        let boards = read_boards(read.lines().skip(2), 5);
        assert_eq!(boards[0].row(0), vec![22, 13, 17, 11, 0]);
        assert_eq!(boards[0].row(1), vec![8, 2, 23, 4, 24]);
        assert_eq!(boards[0].row(2), vec![21, 9, 14, 16, 7]);
    }

    #[test]
    fn board_col() {
        let read = input::read_file(4, 0);
        let boards = read_boards(read.lines().skip(2), 5);
        assert_eq!(boards[0].col(0), vec![22, 8, 21, 6, 1]);
        assert_eq!(boards[0].col(1), vec![13, 2, 9, 10, 12]);
        assert_eq!(boards[0].col(2), vec![17, 23, 14, 3, 20]);
    }
}
