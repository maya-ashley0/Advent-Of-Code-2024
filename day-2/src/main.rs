use std::fs::File;
use std::io::Read;

use nom;
use nom::character::is_newline;

fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let mut input_file_contents = String::new();

    input_file
        .read_to_string(&mut input_file_contents) // TODO stream file with BufReader?
        .expect("couldn't read file into memory");

    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.

    let input = parse_level_reports(&input_file_contents);
    let num_safe: i16 = input
        .iter()
        .map(|report| if determine_safety(report) { 1 } else { 0 })
        .sum();
    println!("num_safe = {}", num_safe);
}

fn determine_safety(report: &Vec<i16>) -> bool {
    let is_negative = (report[1] - report[0]).is_negative(); // TODO don't recompute
    for level_index in 1..report.len() {
        let difference = report[level_index] - report[level_index - 1];
        if (is_negative && difference.is_positive()) || (!is_negative && difference.is_negative()) {
            return false;
        }
        let abs_diff = difference.abs();
        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }
    }
    return true;
}

// TODO add error handling
fn parse_level_reports(input: &str) -> Vec<Vec<i16>> {
    let mut iterator = nom::combinator::iterator(
        input,
        nom::sequence::terminated(parse_levels, nom::character::complete::newline),
    );
    let result = iterator.collect();
    let remaining = iterator.finish();
    if !remaining.expect("error parsing").0.is_empty() {
        panic!("leftover input after parsing");
    }
    result
}

fn parse_levels(input: &str) -> nom::IResult<&str, Vec<i16>> {
    nom::multi::many1(nom::sequence::terminated(
        nom::character::complete::i16,
        nom::combinator::opt(nom::character::complete::space1),
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::determine_safety;

    #[test]
    fn test_determine_safety() {
        assert_eq!(determine_safety(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(determine_safety(&vec![1, 2, 7, 8, 9]), false);
    }
}
