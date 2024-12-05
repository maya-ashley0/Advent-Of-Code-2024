use std::fs::File;
use std::io::Read;

use nom;
use nom::Parser;

fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let mut input_file_contents = String::new();

    input_file
        .read_to_string(&mut input_file_contents) // TODO stream file with BufReader?
        .expect("couldn't read file into memory");

    let input = parse_instructions(&input_file_contents);

    // let sum_of_instructions: u32 = input.iter().map(|i| u32::from(i.0) * u32::from(i.1)).sum();
    // println!(
    //     "sum of valid instructions (part 1): {}",
    //     sum_of_instructions
    // );

    let mut mul_enabled = true;
    let mut sum_of_enabled_instructions = 0;
    input.iter().for_each(|i| match i {
        Instruction::Mul(a, b) => {
            if mul_enabled {
                sum_of_enabled_instructions += u32::from(*a) * u32::from(*b)
            }
        }
        Instruction::Do => {
            mul_enabled = true;
        }
        Instruction::DoNot => {
            mul_enabled = false;
        }
    });
    println!(
        "sum of valid, enabled instructions (part 2): {}",
        sum_of_enabled_instructions
    );
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Mul(u16, u16),
    Do,
    DoNot,
}

// TODO add error handling
fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut iterator = nom::combinator::iterator(input, drop_until);
    let result: Vec<Instruction> = iterator.collect();
    let _remaining = iterator.finish();
    result
}

fn drop_until(input: &str) -> nom::IResult<&str, Instruction> {
    let (input, (_ignore_invalid, valid)) =
        nom::multi::many_till(nom::bytes::complete::take(1usize), parse_instruction)(input)?;
    Ok((input, valid))
}

fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    nom::branch::alt((
        nom::sequence::delimited(
            nom::bytes::complete::tag("mul("),
            parse_pair,
            nom::bytes::complete::tag(")"),
        )
        .map(|(a, b)| Instruction::Mul(a, b)),
        nom::bytes::complete::tag("do()").map(|_| Instruction::Do),
        nom::bytes::complete::tag("don't()").map(|_| Instruction::DoNot),
    ))(input)
}

fn parse_pair(input: &str) -> nom::IResult<&str, (u16, u16)> {
    nom::sequence::separated_pair(
        nom::character::complete::u16,
        nom::bytes::complete::tag(","),
        nom::character::complete::u16,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse_instructions, parse_pair, Instruction};

    #[test]
    fn test_parse_instructions() {
        assert_eq!(vec![Instruction::Mul(1, 2)], parse_instructions("mul(1,2)"));
        assert_eq!(
            vec![Instruction::Mul(1, 2)],
            parse_instructions("amul(1,2)")
        );
        assert_eq!(
            vec![Instruction::Mul(1, 2)],
            parse_instructions("mul(1,2)a")
        );
        assert_eq!(
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ],
            parse_instructions(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
        );
        assert_eq!(
            vec![
                Instruction::Mul(2, 4),
                Instruction::DoNot,
                Instruction::Do,
                Instruction::Mul(8, 5)
            ],
            parse_instructions(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
        );
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(Ok(("", (1, 2))), parse_pair("1,2"));
    }
}
