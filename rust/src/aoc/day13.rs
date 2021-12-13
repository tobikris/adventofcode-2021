use crate::aoc::input;

use std::collections::BTreeMap;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let (dots, cmds) = split(read);
    let mut grid = grid(dots);
    fold(&mut grid, cmds[0]);
    let count = count_dots(&grid);
    format!(
        "Dots visible after completing just the first fold instruction: {}",
        count
    )
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let (dots, cmds) = split(read);
    let mut grid = grid(dots);
    for c in cmds {
        fold(&mut grid, c);
    }
    format!("Code after folding:\n{}", format_grid(&grid))
}

pub fn split(read: String) -> (Vec<(usize, usize)>, Vec<(isize, isize)>) {
    let lines = read.lines();
    let fold = lines.fold(String::new(), |a, l| a + l + "\n");
    let sep = fold.split("\n\n").collect::<Vec<&str>>();

    let dots = sep
        .iter()
        .nth(0)
        .unwrap()
        .lines()
        .map(|l| {
            let s = l.split(",").collect::<Vec<_>>();
            (s[0].parse().unwrap(), s[1].parse().unwrap())
        })
        .collect();

    let cmds = sep
        .iter()
        .nth(1)
        .unwrap()
        .lines()
        .map(|l| {
            let s = l.split("=").collect::<Vec<_>>();
            match s[0] {
                "fold along x" => (s[1].parse().unwrap(), -1),
                "fold along y" => (-1, s[1].parse().unwrap()),
                _ => (-1, -1),
            }
        })
        .collect();

    (dots, cmds)
}

pub fn grid(dots: Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let max_x = dots.iter().map(|i| i.0).max().unwrap();
    let max_y = dots.iter().map(|i| i.1).max().unwrap();
    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for d in dots {
        grid[d.1][d.0] = true;
    }
    grid
}

pub fn format_grid(grid: &Vec<Vec<bool>>) -> String {
    grid.iter()
        .map(|r| {
            r.iter()
                .map(|c| match c {
                    true => "#",
                    false => " ",
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn fold(grid: &mut Vec<Vec<bool>>, fold: (isize, isize)) {
    for (r, row) in grid.clone().iter_mut().enumerate() {
        for (c, col) in row.clone().iter_mut().enumerate() {
            if fold.0 != -1 && c as isize > fold.0 && *col {
                grid[r][(fold.0 - (c as isize - fold.0)) as usize] = true
            }
            if fold.1 != -1 && r as isize > fold.1 && *col {
                grid[(fold.1 - (r as isize - fold.1)) as usize][c] = true
            }
        }
    }
    if fold.0 != -1 {
        for r in grid.iter_mut() {
            r.resize(fold.0 as usize, false);
        }
    }
    if fold.1 != -1 {
        grid.resize(fold.1 as usize, vec![]);
    }
}

pub fn count_dots(grid: &Vec<Vec<bool>>) -> usize {
    grid.into_iter().flatten().filter(|c| **c).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(13, 0);
        let (dots, cmds) = split(read);
        let mut grid = grid(dots);
        fold(&mut grid, cmds[0]);
        assert_eq!(count_dots(&grid), 17);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(13, 0);
    }

    #[test]
    fn split_works() {
        let read = input::read_file(13, 0);
        let (dots, cmds) = split(read);
        assert_eq!(
            dots,
            vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ]
        );
        assert_eq!(cmds, vec![(-1, 7), (5, -1)]);
    }

    #[test]
    fn grid_works() {
        let read = input::read_file(13, 0);
        let (dots, _) = split(read);
        let grid = grid(dots);
        let should = vec![
            vec![
                false, false, false, true, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, true, false, false, false, false, true, false, true,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, true, true, false,
            ],
            vec![
                false, false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, true, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, true, false, false, false, false, false, false, false, false,
            ],
        ];
        assert_eq!(grid, should);
    }

    #[test]
    fn fold_works() {
        let read = input::read_file(13, 0);
        let (dots, cmds) = split(read);
        let mut grid = grid(dots);
        for c in cmds {
            println!("fold: {:?}", c);
            fold(&mut grid, c);
            println!("{}", format_grid(&grid));
        }
        let should = vec![
            vec![true, true, true, true, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, false, false, false, true],
            vec![true, true, true, true, true],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];
        assert_eq!(grid, should);
    }
}
