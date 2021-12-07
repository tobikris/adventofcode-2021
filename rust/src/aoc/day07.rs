use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let mut positions = input::line_as_usize(day, 1).unwrap();
    let (on, fuel) = calc_alignment(&mut positions, |dist| dist);
    format!("Aligning on {} uses the least amount of fuel: {}", on, fuel)
}

pub fn challenge2(day: usize) -> String {
    let mut positions = input::line_as_usize(day, 1).unwrap();
    let (on, fuel) = calc_alignment(&mut positions, |dist| dist * (dist + 1) / 2);
    format!("Aligning on {} uses the least amount of fuel: {}", on, fuel)
}

pub fn calc_alignment(
    positions: &mut Vec<usize>,
    dist: fn(dist: isize) -> isize,
) -> (usize, usize) {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    let mut align = 0;
    let mut fuel = 0;
    for try_align in *min..=*max {
        let try_fuel = positions
            .iter()
            .map(|p| dist((*p as isize - try_align as isize).abs()))
            .sum::<isize>() as usize;
        if fuel == 0 || try_fuel < fuel {
            align = try_align;
            fuel = try_fuel;
        }
    }
    (align, fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| dist);
        assert_eq!(on, 2);
        assert_eq!(fuel, 37);
    }

    #[test]
    fn challenge2() {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| (dist * (dist + 1) / 2));
        assert_eq!(on, 5);
        assert_eq!(fuel, 168);
    }

    #[test]
    fn calc_alignment_works() {
        let positions = vec![1];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| dist);
        assert_eq!(on, 1);
        assert_eq!(fuel, 0);

        let positions = vec![1, 3];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| dist);
        assert_eq!(on, 1);
        assert_eq!(fuel, 2);
    }

    #[test]
    fn calc_alignment2_works() {
        let positions = vec![1];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| dist * (dist + 1) / 2);
        assert_eq!(on, 1);
        assert_eq!(fuel, 0);

        let positions = vec![1, 3];
        let (on, fuel) = calc_alignment(&mut positions.clone(), |dist| dist * (dist + 1) / 2);
        assert_eq!(on, 2);
        assert_eq!(fuel, 2);
    }
}
