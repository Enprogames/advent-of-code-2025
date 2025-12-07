use anyhow::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");
// TODO: Get big boy input
// const INPUT_FILE_BIG_BOY: &str = concatcp!("input/", "bigboy", DAY, ".txt");

const TEST: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

enum Move {
    Left(isize),
    Right(isize),
}

fn parse_moves_from_file<R: BufRead>(mut reader: R) -> Result<Vec<Move>> {
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer).context("Failed to read file as bytes")?;

    let result = buffer.par_split(|&c| c == b'\n')
        .filter(|&mov_str| !mov_str.is_empty())
        .map(|mov_str| {
            let (rotation_count, _) = parse_int_ascii(&mov_str[1..]);

            if mov_str[0] == b'L' {
                Move::Left(rotation_count as isize)
            } else {
                Move::Right(rotation_count as isize)
            }
        })
        .collect::<Vec<Move>>();

    Ok(result)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let moves = parse_moves_from_file(reader)?;

    let dial_start = 50;
    let (_, result) = moves.iter().fold((dial_start, 0), |(running_sum, zero_count), cur_move| {
        let new_pos = match cur_move {
            Move::Left(val) => {
                (running_sum - val).rem_euclid(100)  // Mathematical modulo operator
            },
            Move::Right(val) => {
                (running_sum + val).rem_euclid(100)  // Mathematical modulo operator
            }
        };

        let new_zero_count =  if new_pos == 0 {
            zero_count + 1
        } else {
            zero_count
        };

        (new_pos, new_zero_count)
    });

    Ok(result)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 2 of the puzzle
    Ok(0)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

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
