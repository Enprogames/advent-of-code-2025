use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
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
"#; // TODO: Enter test input

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 1 of the puzzle

    fn count_both_sides_equal_occurences(start: &str, end: &str) -> usize {
        let count = 0;
        
        // Skip this if both sides are odd-length and no even-length values exist in-between.
        // An odd-length number cannot have both sides equal.
        if start.len() == end.len() && end.len() % 2 == 1 {
            return 0
        }

        let (start_val, start_len) = parse_int_ascii(start.as_bytes());
        let (end_val, end_len) = parse_int_ascii(end.as_bytes());

        for val in start..=end {
            let current_length_reference = 1;
            let current_length = 0;

            if 
        }

        count
    }
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<String>()
        .split(",")
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|range_str|  // Get left and right items in the string
        {
            let range_split = range_str.split("-");
            let left = range_split.next()?;
            let right = range_split.next()?;

            count_both_sides_equal_occurences(left, right)
        })
        .sum();

    let answer = reader.lines().flatten().count();
    Ok(answer)
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
