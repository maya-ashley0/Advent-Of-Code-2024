use std::fs::File;
use std::io::Read;

use nom;

// this pseudocode should be possible, but I don't feel like writing it:
// ```text
// list_1.sort()
// sort list_2 and count the number of items while sorting, inserting into list_2_count_map
// iterate over list_1 and calculate sum_of_products
// iterate over list_1 + list_2, calculating summed_differences
// ```
// this should make one iteration of list_2 (on line `29`) unnecessary
fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let mut input_file_contents = String::new();
    let mut list_2_count_map = std::collections::HashMap::new();

    input_file
        .read_to_string(&mut input_file_contents) // TODO stream file with BufReader?
        .expect("couldn't read file into memory");

    let (mut list_1, mut list_2) = decimal_pair_newline(&input_file_contents);

    list_1.sort();
    list_2.sort();

    // calculate part 2 first so we don't have to clone:
    list_2
        .iter_mut()
        .for_each(|id| *list_2_count_map.entry(id).or_insert(0) += 1);

    let sum_of_products: u64 = list_1
        .iter()
        .map(|id| u64::from(*id) * list_2_count_map.get(id).unwrap_or(&0))
        .sum();

    let summed_differences: u32 = std::iter::zip(list_1, list_2)
        .map(|(id1, id2)| id2.abs_diff(id1))
        .sum();

    println!("(part 1) summed_differences is {}", summed_differences);
    println!("(part 2) sum_of_products is {}", sum_of_products);
}

// TODO add error handling
fn decimal_pair_newline(input: &str) -> (Vec<u32>, Vec<u32>) {
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

fn decimal_whitespace_decimal(input: &str) -> nom::IResult<&str, (u32, u32)> {
    nom::sequence::separated_pair(
        nom::character::complete::u32,
        nom::character::complete::space1,
        nom::character::complete::u32,
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
        let a: Vec<u32> = vec![1, 3, 5, 7, 9];
        let b: Vec<u32> = vec![2, 4, 6, 8, 10];
        assert_eq!(
            decimal_pair_newline("1   2\n3   4\n5   6\n7   8\n9   10\n"),
            (a, b)
        );
    }
}
