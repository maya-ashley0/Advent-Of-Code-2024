use std::io::Read;
use std::{fs::File, io::BufReader, str::FromStr};

use nom;

fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let mut input_file_contents = String::new();

    input_file
        .read_to_string(&mut input_file_contents)
        .expect("couldn't read file into memory");
}

fn decimal_pair_newline<T: FromStr>(input: &str) -> nom::IResult<&str, (Vec<T>, Vec<T>)> {
    let mut iterator = nom::combinator::iterator(
        input,
        nom::sequence::terminated(
            decimal_whitespace_decimal,
            nom::character::complete::newline,
        ),
    );
    let (a, b) = iterator.unzip();
    let remaining = iterator.finish();
    Ok((remaining.unwrap().0, (a, b)))
}

fn decimal_whitespace_decimal<T: FromStr>(input: &str) -> nom::IResult<&str, (T, T)> {
    nom::sequence::separated_pair(decimal, nom::character::complete::space1, decimal)(input)
}

// Taken from https://docs.rs/nom/latest/nom/recipes/index.html#decimal
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
    use crate::decimal_whitespace_decimal;

    #[test]
    fn test_decimal_whitespace_decimal() {
        assert_eq!(decimal_whitespace_decimal("3   4"), Ok(("", (3, 4))));
    }

    #[test]
    fn test_decimal_pair_newline() {
        let a: Vec<u16> = vec![3, 5];
        let b: Vec<u16> = vec![4, 6];
        assert_eq!(
            decimal_whitespace_decimal("3   4\n5   6"),
            Ok(("", (a, b)))
        );
    }
}
