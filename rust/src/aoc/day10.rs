use crate::aoc::input;

extern crate pest;
use pest::Parser;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let score: usize = score_illegal(&input::read_file(day, 1));
    format!(
        "Total syntax error score for illegal character errors: {}",
        score
    )
}

pub fn challenge2(day: usize) -> String {
    let score: usize = score_incomplete(&input::read_file(day, 1));
    format!(
        "Total syntax error score for incomplete character errors: {}",
        score
    )
}

#[derive(pest_derive::Parser)]
#[grammar = "aoc/day10.pest"]
pub struct NavParser;

pub fn score_illegal(read: &str) -> usize {
    read.lines()
        .map(|l| {
            if let Err(err) = NavParser::parse(Rule::line, l) {
                if let pest::error::InputLocation::Pos(pos) = err.location {
                    if pos < l.len() {
                        return scoring_illegal(l.chars().nth(pos).unwrap());
                    }
                }
            }
            0
        })
        .sum()
}

pub fn scoring_illegal(symbol: char) -> usize {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn generate_one(incomplete: &mut String, score: &mut usize) -> (String, usize) {
    if let Err(err) = NavParser::parse(Rule::line, &incomplete) {
        if let pest::error::InputLocation::Pos(pos) = err.location {
            if pos >= incomplete.len() {
                if let pest::error::ErrorVariant::ParsingError {
                    positives,
                    negatives: _,
                } = err.variant
                {
                    *score *= 5;
                    *score += scoring_incomplete(positives[0]);
                    incomplete.push(char_incomplete(positives[0]));
                    return generate_one(incomplete, score);
                }
            }
        }
    }
    (incomplete.clone(), *score)
}

pub fn score_incomplete(read: &str) -> usize {
    let mut scores = read
        .lines()
        .filter_map(|l| {
            if let Err(err) = NavParser::parse(Rule::line, l) {
                if let pest::error::InputLocation::Pos(pos) = err.location {
                    if pos >= l.len() {
                        let mut incomplete = l.to_owned();
                        let (_, score) = generate_one(&mut incomplete, &mut 0);
                        return Some(score);
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.cmp(a));
    *scores.iter().nth(scores.len() / 2).unwrap()
}

pub fn scoring_incomplete(symbol: Rule) -> usize {
    match symbol {
        Rule::parens_r => 1,
        Rule::brackets_r => 2,
        Rule::braces_r => 3,
        Rule::tags_r => 4,
        _ => 0,
    }
}

pub fn char_incomplete(symbol: Rule) -> char {
    match symbol {
        Rule::parens_r => ')',
        Rule::brackets_r => ']',
        Rule::braces_r => '}',
        Rule::tags_r => '>',
        _ => ' ',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let score: usize = score_illegal(&input::read_file(10, 0));
        assert_eq!(score, 26397);
    }

    #[test]
    fn challenge2() {
        let score: usize = score_incomplete(&input::read_file(10, 0));
        assert_eq!(score, 288957);
    }

    #[test]
    fn parsing_works() {
        assert_eq!(NavParser::parse(Rule::line, "{}").is_ok(), true);
        assert_eq!(NavParser::parse(Rule::line, "{{}}").is_ok(), true);
        assert_eq!(NavParser::parse(Rule::line, "{}{}").is_ok(), true);
        assert_eq!(NavParser::parse(Rule::line, "{{{}").is_ok(), false);
        assert_eq!(NavParser::parse(Rule::line, "[]").is_ok(), true);
        assert_eq!(NavParser::parse(Rule::line, "[}").is_ok(), false);
    }
}
