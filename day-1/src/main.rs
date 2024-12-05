use std::io::Read;
use std::{fs::File, str::FromStr};

use nom;

fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let mut input_file_contents = String::new();

    input_file
        .read_to_string(&mut input_file_contents) // TODO stream file with BufReader?
        .expect("couldn't read file into memory");

    let (mut list_1, mut list_2) = decimal_pair_newline::<u32>(&input_file_contents);

    list_1.sort();
    list_2.sort();

    let summed_differences: u32 = std::iter::zip(list_1, list_2)
        .map(|(id1, id2)| id2.abs_diff(id1))
        .sum();

    println!("(part 1) summed_differences is {}", summed_differences);
}

// TODO add error handling
fn decimal_pair_newline<T: FromStr>(input: &str) -> (Vec<T>, Vec<T>) {
    let mut iterator = nom::combinator::iterator(
        input,
        nom::sequence::terminated(
            decimal_whitespace_decimal,
            nom::character::complete::newline,
        ),
    );
    let (a, b) = iterator.unzip();
    let remaining = iterator.finish(); // this panics if iterator was already consumed
    if !remaining.expect("error parsing").0.is_empty() {
        panic!("leftover input after parsing");
    }
    (a, b)
}

fn decimal_whitespace_decimal<T: FromStr>(input: &str) -> nom::IResult<&str, (T, T)> {
    nom::sequence::separated_pair(decimal, nom::character::complete::space1, decimal)(input)
}

// Taken from https://docs.rs/nom/latest/nom/recipes/index.html#decimal
// replace with nom::character::complete::T ? how to genericize cleanly without exposing nom types?
fn decimal<T: FromStr>(input: &str) -> nom::IResult<&str, T> {
    nom::combinator::map_res(
        nom::combinator::recognize(nom::multi::many1(nom::sequence::terminated(
            nom::character::complete::one_of("0123456789"),
            nom::multi::many0(nom::character::complete::char('_')),
        ))),
        |num_str| T::from_str(num_str),
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{decimal_pair_newline, decimal_whitespace_decimal};

    #[test]
    fn test_decimal_whitespace_decimal() {
        assert_eq!(decimal_whitespace_decimal("3   4"), Ok(("", (3, 4))));
    }

    #[test]
    fn test_decimal_pair_newline() {
        let a: Vec<u8> = vec![1, 3, 5, 7, 9];
        let b: Vec<u8> = vec![2, 4, 6, 8, 10];
        assert_eq!(
            decimal_pair_newline("1   2\n3   4\n5   6\n7   8\n9   10\n"),
            (a, b)
        );
    }
}
