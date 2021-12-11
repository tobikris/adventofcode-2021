use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let mut grid = read_grid(day, 1);
    let flashes = do_step(&mut grid, 100);
    format!("Total flashes after step 100: {}", flashes)
}

pub fn challenge2(day: usize) -> String {
    let mut grid = read_grid(day, 1);
    let steps = do_sync(&mut grid);
    format!("Synced after step: {}", steps)
}

pub fn read_grid(day: usize, challenge: usize) -> Vec<Vec<usize>> {
    input::read_file(day, challenge)
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn flash(grid: &mut Vec<Vec<usize>>, row: isize, col: isize) {
    if row < 0
        || row >= (grid.len() as isize)
        || col < 0
        || col >= (grid[row as usize].len() as isize)
        || grid[row as usize][col as usize] > 9
    {
        return;
    }
    grid[row as usize][col as usize] += 1;
    if grid[row as usize][col as usize] == 10 {
        flash(grid, row - 1, col - 1);
        flash(grid, row - 1, col);
        flash(grid, row - 1, col + 1);
        flash(grid, row, col - 1);
        flash(grid, row, col + 1);
        flash(grid, row + 1, col - 1);
        flash(grid, row + 1, col);
        flash(grid, row + 1, col + 1);
    }
}

pub fn do_step(grid: &mut Vec<Vec<usize>>, steps: usize) -> usize {
    let mut flashes = 0;
    for _ in 0..steps {
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                flash(grid, r as isize, c as isize);
            }
        }
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 10 {
                    flashes += 1;
                    grid[r][c] = 0;
                }
            }
        }
    }
    flashes
}

pub fn do_sync(grid: &mut Vec<Vec<usize>>) -> usize {
    let count = grid.len() * grid[0].len();
    let mut steps = 0;
    loop {
        let flashes = do_step(grid, 1);
        steps += 1;
        if flashes == count {
            break;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let mut grid = read_grid(11, 0);
        let mut flashes = do_step(&mut grid, 1);
        assert_eq!(flashes, 0);
        assert_eq!(
            grid,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
        );
        flashes += do_step(&mut grid, 1);
        assert_eq!(flashes, 35);
        assert_eq!(
            grid,
            vec![
                vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
            ]
        );
        flashes += do_step(&mut grid, 8);
        assert_eq!(flashes, 204);
        flashes += do_step(&mut grid, 90);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn challenge2() {
        let mut grid = read_grid(11, 0);
        let steps = do_sync(&mut grid);
        assert_eq!(steps, 195);
    }

    #[test]
    fn do_step_works() {
        let mut grid = vec![vec![8, 7]];
        let flashes = do_step(&mut grid, 1);
        assert_eq!(flashes, 0);
        assert_eq!(grid, vec![vec![9, 8],]);
        let flashes = do_step(&mut grid, 1);
        assert_eq!(flashes, 2);
        assert_eq!(grid, vec![vec![0, 0],]);

        let mut grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 9, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let flashes = do_step(&mut grid, 1);
        assert_eq!(flashes, 1);
        assert_eq!(
            grid,
            vec![
                vec![2, 2, 2, 1],
                vec![2, 0, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 1],
            ]
        );
    }
}
