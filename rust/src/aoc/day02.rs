use crate::aoc::input;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let steps = input::read_file(day, 1);
    let lines = steps.lines().collect();
    let (horiz, depth) = plan_course_with_aim(&lines, false);
    return format!("Horizontal {} * depth {} = {}", horiz, depth, horiz * depth);
}

pub fn challenge2(day: usize) -> String {
    let steps = input::read_file(day, 1);
    let lines = steps.lines().collect();
    let (horiz, depth) = plan_course_with_aim(&lines, true);
    return format!("Horizontal {} * depth {} = {}", horiz, depth, horiz * depth);
}

fn plan_course_with_aim(measurements: &Vec<&str>, include_aim: bool) -> (usize, usize) {
    let mut horiz: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    measurements.iter().for_each(|l| {
        let mut line = l.split(' ');
        let cmd = line.next();
        let x = line.next().unwrap().parse::<usize>().unwrap();
        match cmd {
            Some("forward") => {
                horiz += x;
                if include_aim {
                    depth += aim * x;
                }
            }
            Some("down") => aim += x,
            Some("up") => aim -= x,
            Some(&_) => {}
            None => {}
        }
    });
    if include_aim {
        aim = depth;
    }
    (horiz, aim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let steps = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let (horiz, depth) = plan_course_with_aim(&steps, false);
        assert_eq!(horiz, 15);
        assert_eq!(depth, 10);
    }

    #[test]
    fn challenge2() {
        let steps = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let (horiz, depth) = plan_course_with_aim(&steps, true);
        assert_eq!(horiz, 15);
        assert_eq!(depth, 60);
    }
}
