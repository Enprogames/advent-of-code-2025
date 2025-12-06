#![allow(non_snake_case)]

use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2025::*;
use rayon::prelude::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
// const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"#;


fn is_eqSideNum(val: &[u8]) -> bool {
    if val.len() % 2 == 1 {
        false
    }
    else if val.len() == 0 {
        true
    } else {
        let split_index = val.len() / 2 - 1;
        let left_half = &val[0..split_index];
        let right_half = &val[split_index+1..val.len()-1];

        left_half == right_half
    }
}

/// For a given value (string of bytes), get the previous number
/// whose string representation is the same on both sides
fn get_cur_or_prev_eqSideNum_half(val: &[u8]) -> Result<usize> {
    let val_len = val.len();

    if is_eqSideNum(val) {
        let (result, _) = parse_int_ascii(&val[..val_len / 2]);
        return Ok(result)
    }

    if val_len % 1 == 0 {
        let exp = ((val_len as u32) + 1) / 2 - 1;
        let result = 10_usize.checked_pow(exp).ok_or_else(|| anyhow!("Exponent calculation caused overflow"))?;

        return Ok(result) // e.g. 123 -> 1010
    }

    let (lh, _) = parse_int_ascii(&val[..val_len / 2]);  // Left half
    let (rh, _) = parse_int_ascii(&val[val_len / 2..]);  // Right half

    if lh > rh {  // e.g. 9998 -> 9999
        return Ok(lh)
    }

    Ok(lh + 1)  //  lh < rh e.g. 1234 -> 1313
}

/// For a given number, which must be 4 digits long (give an error otherwise),
/// find the next number where both sides are repeated (e.g. 9898, 9999, 1313, 12341234, etc.).
/// So if we were given 1024, the answer would be 1111. And if we were given 1000, we'd give 99.
fn get_cur_or_next_eqSideNum_half(val: &[u8]) -> Result<usize> {
    let val_len = val.len();

    if is_eqSideNum(val) {
        let (result, _) = parse_int_ascii(&val[..val_len / 2]);
        return Ok(result)
    }

    dbg!(val);

    if val_len % 1 == 0 {
        let exp = ((val_len as u32) - 1) / 2 - 1;
        let result = 10_usize.checked_pow(exp).ok_or_else(|| anyhow!("Exponent calculation caused overflow"))?;

        return Ok(result - 1) // e.g. 123 -> 10^2 = (100 - 1) = 99
    }

    let (lh, _) = parse_int_ascii(&val[..val_len / 2]);  // Left half
    let (rh, _) = parse_int_ascii(&val[val_len / 2..]);  // Right half

    dbg!(lh);
    dbg!(rh);

    if lh > rh {  // e.g. 9998 -> 9898
        return Ok(lh - 1)
    }

    Ok(lh)  //  lh < rh e.g. 1234 -> 1212
}

fn count_both_sides_eqSideNum_occurences(start: &[u8], end: &[u8]) -> Result<usize> {
    // let mut count = 0;

    // Skip this if both sides are odd-length and no even-length values exist in-between.
    // An odd-length number cannot have both sides equal.
    if start.len() == end.len() && end.len() % 2 == 1 {
        return Ok(0)
    }

    let start_half_val = get_cur_or_prev_eqSideNum_half(start)?;

    let end_half_val = get_cur_or_next_eqSideNum_half(end)?;

    let result = if start_half_val > end_half_val {
        0
    } else if start_half_val == end_half_val {
        1
    } else {
        end_half_val - start_half_val
    };

    Ok(result)
}

fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer).context("Failed to read file as bytes");

    let result = buffer
        .par_split(|&b| b == b',' || b == b'\n' || b == b'\r')
        .map(|range_str|  // Get left and right items in the string
        {
            let mut elements = range_str.splitn(2, |&b| b == b'-');
            let left = elements.next().context("Missing left side")?;
            let right = elements.next().context("Missing right side")?;

            count_both_sides_eqSideNum_occurences(left, right)
        })
        .sum::<Result<usize>>();

    result
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 2 of the puzzle
    Ok(0)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // TODO: Uncomment for big boy result
    // let result = time_snippet!(part1(
    //     BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    // )?);
    // println!("Result (big boy) = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);

    // TODO: Uncomment for big boy result
    // let result = time_snippet!(part2(
    //     BufReader::new(File::open(INPUT_FILE_BIG_BOY)?)
    // )?);
    // println!("Result (big boy) = {}", result);
    //endregion

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"1011".as_ref(), 10)]
    #[case(b"0000".as_ref(), 0)]
    #[case(b"123".as_ref(), 99)]
    fn test_get_cur_or_prev_eqSideNum_half(#[case] input: &[u8], #[case] expected: usize) { // 2. Expect usize
        let result = get_cur_or_prev_eqSideNum_half(input);

        // 3. Unwrap the result to check validity, then compare the value
        assert_eq!(result.unwrap(), expected); 
    }

    #[rstest]
    #[case(b"1011".as_ref(), 11)]
    #[case(b"0000".as_ref(), 0)]
    #[case(b"123".as_ref(), 10)]
    fn test_get_cur_or_next_eqSideNum_half(#[case] input: &[u8], #[case] expected: usize) {
        let result = get_cur_or_next_eqSideNum_half(input); 

        assert_eq!(result.unwrap(), expected);
    }

    #[rstest]
    #[case(b"99-101".as_ref(), 1)]
    #[case(b"99-9999".as_ref(), 90)]
    #[case(b"99-101,99-9999".as_ref(), 91)]
    fn test_count_both_sides_eqSideNum_occurences(#[case] input: &[u8], #[case] expected: usize) {
        let result = part1(input);

        assert_eq!(result.unwrap(), expected);
    }
}
