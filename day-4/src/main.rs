use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::slice::ChunksExact;

fn main() {
    let mut input_file = File::open("src/input.txt").expect("couldn't open file");
    let input_file_len = input_file
        .metadata()
        .expect("unable to get input file metadata")
        .len()
        .try_into()
        .expect("input file too big!");
    let mut input_file_contents = Vec::with_capacity(input_file_len);
    input_file_contents.resize(input_file_len, 0);

    input_file
        .read(&mut input_file_contents[..]) // TODO stream file with BufReader?
        .expect("couldn't read file into memory");

    let length_of_line = input_file_contents
            .iter()
            .position(|c| *c == Char::LineFeed as u8)
            .expect("unable to find a newline") // TODO handle single-line files
            + 1;
    let input_lines = input_file_contents.chunks_exact(length_of_line);
    if !input_lines.remainder().is_empty() {
        panic!("leftover input after splitting input lines");
    }

    let num_hits_pt_1: u32 =
        find_positions_of_xmas(&input_file_contents, input_lines.clone(), length_of_line)
            .into_values()
            .map(|count| u32::from(count))
            .sum();
    println!("found XMAS {} times", num_hits_pt_1);

    let num_hits_pt_2 =
        find_positions_of_mas_in_x(&input_file_contents, input_lines, length_of_line);
    println!("found MAS in X {} times", num_hits_pt_2);
}

// std::ascii::Char isn't stable yet, so here's what we need from it:
#[repr(u8)]
enum Char {
    LineFeed = 10,
    CapitalA = 65,
    CapitalM = 77,
    CapitalS = 83,
    CapitalX = 88,
}

fn find_positions_of_mas_in_x(
    input_file_contents: &Vec<u8>,
    input_lines: ChunksExact<'_, u8>,
    length_of_line: usize,
) -> usize {
    let mut results = 0;
    let mut line_id = 0;
    let num_lines = input_lines.len();
    for line in input_lines {
        // don't use enumerate, since it consumes the iter for ChunksExact
        let mut c_id = 0;
        for c in line {
            if *c == Char::CapitalA as u8 {
                if input_file_contents.num_mas_in_x_at(num_lines, length_of_line, line_id, c_id) {
                    results += 1;
                }
            }
            c_id += 1;
        }
        line_id += 1;
    }
    results
}

fn find_positions_of_xmas(
    input_file_contents: &Vec<u8>,
    input_lines: ChunksExact<'_, u8>,
    length_of_line: usize,
) -> HashMap<(usize, usize), u8> {
    let mut results = HashMap::new();
    let mut line_id = 0;
    let num_lines = input_lines.len();
    for line in input_lines {
        // don't use enumerate, since it consumes the iter for ChunksExact
        let mut c_id = 0;
        for c in line {
            if *c == Char::CapitalX as u8 {
                let hits =
                    input_file_contents.num_xmas_at(num_lines, length_of_line, line_id, c_id);
                if hits > 0 {
                    results.insert((line_id, c_id), hits);
                }
            }
            c_id += 1;
        }
        line_id += 1;
    }
    results
}

trait CountXmasOccurrences {
    fn num_mas_in_x_at(
        &self,
        num_lines: usize,
        length_of_line: usize,
        line_id: usize,
        c_id: usize,
    ) -> bool;

    fn look_for_mas_in_x(
        &self,
        length_of_line: usize,
        index: usize,
        c_direction: isize,
        line_direction: isize,
    ) -> bool;

    fn num_xmas_at(
        &self,
        num_lines: usize,
        length_of_line: usize,
        line_id: usize,
        c_id: usize,
    ) -> u8;

    fn look_for_mas(
        &self,
        length_of_line: usize,
        index: usize,
        c_direction: isize,
        line_direction: isize,
    ) -> bool;
}

impl CountXmasOccurrences for Vec<u8> {
    // should have made a new wrapping type to hold extra state
    fn look_for_mas_in_x(
        &self,
        length_of_line: usize,
        index: usize,
        c_direction: isize,
        line_direction: isize,
    ) -> bool {
        let index = index as isize;
        let length_of_line = length_of_line as isize;
        self[(index - 1 * c_direction + length_of_line * -1 * line_direction) as usize]
            == Char::CapitalM as u8
            // we already know there's an A here, because that's why we're looking here:
            //&& self[(index + 0 * c_direction + length_of_line * 0 * line_direction) as usize]
            //    == Char::CapitalA as u8
            && self[(index + 1 * c_direction + length_of_line * 1 * line_direction) as usize]
                == Char::CapitalS as u8
    }

    fn num_mas_in_x_at(
        &self,
        num_lines: usize,
        length_of_line: usize,
        line_id: usize,
        c_id: usize,
    ) -> bool {
        let index = calculate_index(length_of_line, line_id, c_id);
        const REMAINING_LEN: usize = 1; // length of either "M" or "S"

        let room_plus_x = length_of_line - c_id > REMAINING_LEN;
        let room_minus_x = c_id >= REMAINING_LEN;
        let room_plus_y = num_lines - line_id > REMAINING_LEN;
        let room_minus_y = line_id >= REMAINING_LEN;

        if !(room_plus_x && room_plus_y && room_minus_x && room_minus_y) {
            return false;
        }

        return (self.look_for_mas_in_x(length_of_line, index, 1, 1)
            || self.look_for_mas_in_x(length_of_line, index, -1, -1))
            && (self.look_for_mas_in_x(length_of_line, index, 1, -1)
                || self.look_for_mas_in_x(length_of_line, index, -1, 1));
    }

    fn look_for_mas(
        &self,
        length_of_line: usize,
        index: usize,
        c_direction: isize,
        line_direction: isize,
    ) -> bool {
        let index = index as isize;
        let length_of_line = length_of_line as isize;
        self[(index + 1 * c_direction + length_of_line * 1 * line_direction) as usize]
            == Char::CapitalM as u8
            && self[(index + 2 * c_direction + length_of_line * 2 * line_direction) as usize]
                == Char::CapitalA as u8
            && self[(index + 3 * c_direction + length_of_line * 3 * line_direction) as usize]
                == Char::CapitalS as u8
    }

    fn num_xmas_at(
        &self,
        num_lines: usize,
        length_of_line: usize,
        line_id: usize,
        c_id: usize,
    ) -> u8 {
        let index = calculate_index(length_of_line, line_id, c_id);
        let mut results = 0;
        const REMAINING_LEN: usize = 3; // length of "MAS"

        let room_plus_x = length_of_line - c_id > REMAINING_LEN;
        let room_minus_x = c_id >= REMAINING_LEN;
        let room_plus_y = num_lines - line_id > REMAINING_LEN;
        let room_minus_y = line_id >= REMAINING_LEN;

        if room_plus_x && self.look_for_mas(length_of_line, index, 1, 0) {
            results += 1;
        }
        if room_minus_x && self.look_for_mas(length_of_line, index, -1, 0) {
            results += 1;
        }
        if room_plus_y && self.look_for_mas(length_of_line, index, 0, 1) {
            results += 1;
        }
        if room_minus_y && self.look_for_mas(length_of_line, index, 0, -1) {
            results += 1;
        }
        if room_plus_x && room_plus_y && self.look_for_mas(length_of_line, index, 1, 1) {
            results += 1;
        }
        if room_plus_x && room_minus_y && self.look_for_mas(length_of_line, index, 1, -1) {
            results += 1;
        }
        if room_minus_x && room_plus_y && self.look_for_mas(length_of_line, index, -1, 1) {
            results += 1;
        }
        if room_minus_x && room_minus_y && self.look_for_mas(length_of_line, index, -1, -1) {
            results += 1;
        }

        results
    }
}

fn calculate_index(length_of_line: usize, line_id: usize, c_id: usize) -> usize {
    line_id * length_of_line + c_id
}

#[cfg(test)]
mod tests {
    use crate::Char;
    use crate::CountXmasOccurrences;

    #[test]
    fn test_search_xmas_does_not_panic() {
        let input = vec![Char::CapitalX as u8; 5 * 5];
        let mut count = 0;
        for line in 0..5 {
            for char in 0..5 {
                count += input.num_xmas_at(5, 5, line, char);
            }
        }
        assert_eq!(0, count);
    }

    #[test]
    fn test_search_mas_does_not_panic() {
        let input = vec![Char::CapitalM as u8; 5 * 5];
        let mut count = 0;
        for line in 0..5 {
            for char in 0..5 {
                if input.num_mas_in_x_at(5, 5, line, char) {
                    count += 1;
                }
            }
        }
        assert_eq!(0, count);
    }
}
