use crate::aoc::input;

use std::collections::HashMap;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let h = read_heightmap(read);
    format!(
        "Sum of the risk levels of all low points: {}",
        h.calc_risk_sum()
    )
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let mut h = read_heightmap(read);
    h.flood_fill();
    format!(
        "Multiply the three largest basin sizes: {}",
        h.multiply_largest_basin_sizes()
    )
}

#[derive(Debug, Clone)]
pub struct Heightmap {
    grid: Vec<Vec<usize>>,
}

impl Heightmap {
    pub fn calc_risk_sum(self) -> usize {
        let mut risk_sum = 0;
        for (row, r) in self.grid.iter().enumerate() {
            for (col, c) in r.iter().enumerate() {
                if let Some(true) = self.clone().is_low(row, col) {
                    risk_sum += 1 + c
                }
            }
        }
        risk_sum
    }
    pub fn is_low(self, row: usize, col: usize) -> Option<bool> {
        if let Some(val) = self.grid.get(row)?.get(col) {
            if *val == 9 {
                return Some(false);
            }
            if row > 0 {
                if let Some(row) = self.grid.get(row - 1) {
                    if val >= &row[col] {
                        return Some(false);
                    }
                }
            }
            if let Some(row) = self.grid.get(row + 1) {
                if val >= &row[col] {
                    return Some(false);
                }
            }
            if col > 0 {
                if let Some(col) = self.grid[row].get(col - 1) {
                    if val >= col {
                        return Some(false);
                    }
                }
            }
            if let Some(col) = self.grid[row].get(col + 1) {
                if val >= col {
                    return Some(false);
                }
            }
            return Some(true);
        }
        None
    }
    pub fn flood_fill(&mut self) {
        let mut counter = 100;
        for (row, r) in self.clone().grid.iter().enumerate() {
            for (col, _) in r.iter().enumerate() {
                self.flood_step(row, col, counter);
                counter += 1;
            }
        }
    }
    fn flood_step(&mut self, row: usize, col: usize, color: usize) {
        match self.field_mut(row, col) {
            Some(val) => match val {
                0..=8 => {
                    *val = color;
                    if row > 0 {
                        self.flood_step(row - 1, col, color);
                    }
                    self.flood_step(row + 1, col, color);
                    if col > 0 {
                        self.flood_step(row, col - 1, color);
                    }
                    self.flood_step(row, col + 1, color);
                }
                _ => {}
            },
            None => {}
        }
    }
    fn field_mut(&mut self, row: usize, col: usize) -> Option<&mut usize> {
        return self.grid.get_mut(row)?.get_mut(col);
    }
    pub fn multiply_largest_basin_sizes(&self) -> usize {
        let mut basins = HashMap::new();
        self.grid
            .iter()
            .map(|r| r.iter().filter(|&&c| c > 9).collect::<Vec<_>>())
            .flatten()
            .for_each(|b| {
                *basins.entry(b).or_insert(0) += 1;
            });
        let mut basins = basins.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        basins.sort_by(|a, b| b.cmp(a));
        let mut product = 1;
        for b in basins.windows(3).nth(0).unwrap() {
            product *= b;
        }
        product
    }
}

pub fn read_heightmap(lines: String) -> Heightmap {
    Heightmap {
        grid: lines
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(9, 0);
        let h = read_heightmap(read);
        let sum = h.calc_risk_sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(9, 0);
        let mut h = read_heightmap(read);
        h.flood_fill();
        assert_eq!(h.multiply_largest_basin_sizes(), 1134);
    }

    #[test]
    fn calc_risk_sum() {
        let h = Heightmap {
            grid: vec![vec![1]],
        };
        assert_eq!(h.calc_risk_sum(), 2);

        let h = Heightmap {
            grid: vec![vec![1, 2]],
        };
        assert_eq!(h.calc_risk_sum(), 2);

        let h = Heightmap {
            grid: vec![vec![1, 2, 1]],
        };
        assert_eq!(h.calc_risk_sum(), 4);

        let h = Heightmap {
            grid: vec![vec![1], vec![2], vec![1]],
        };
        assert_eq!(h.calc_risk_sum(), 4);
    }

    #[test]
    fn flood_fill() {
        let mut h = Heightmap {
            grid: vec![vec![1]],
        };
        h.flood_fill();
        assert_eq!(h.grid, vec![vec![100]]);

        let mut h = Heightmap {
            grid: vec![vec![1, 2]],
        };
        h.flood_fill();
        assert_eq!(h.grid, vec![vec![100, 100]]);

        let mut h = Heightmap {
            grid: vec![vec![1, 2, 1]],
        };
        h.flood_fill();
        assert_eq!(h.grid, vec![vec![100, 100, 100]]);

        let mut h = Heightmap {
            grid: vec![vec![1], vec![2], vec![1]],
        };
        h.flood_fill();
        assert_eq!(h.grid, vec![vec![100], vec![100], vec![100]]);

        let mut h = Heightmap {
            grid: vec![vec![1, 9, 1]],
        };
        h.flood_fill();
        assert_eq!(h.grid, vec![vec![100, 9, 102]]);
    }
}
