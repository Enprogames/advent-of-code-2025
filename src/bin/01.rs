use anyhow::{Context, Result, anyhow};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const INPUT_PATH: &str = "input/01.txt";

fn main() -> Result<()> {
    println!("--- Day 01 ---");

    // Part 1
    let start = Instant::now();

    let mut reader1 = reader_from_file_path(INPUT_PATH)?;
    let p1 = part1(&mut reader1)?;
    println!("Part 1: {} ({:.2?})", p1, start.elapsed());

    // Part 2
    let start = Instant::now();
    let mut reader2 = reader_from_file_path(INPUT_PATH)?;
    let p2 = part2(&mut reader2)?;
    println!("Part 2: {} ({:.2?})", p2, start.elapsed());

    Ok(())
}

/// Find number of times the lock stops at 0 while making forwards and reverse rotations
/// from 0 to 99.
fn part1(reader: &mut impl BufRead) -> Result<usize> {

    let mut result = 0;
    let mut position: i32 = 50;
    let mut buf = Vec::new();

    loop {
        // Continually clearing keeps this loop zero-allocation
        buf.clear();

        // Read the line
        let bytes_read = reader.read_until(b'\n', &mut buf)?;

        if bytes_read == 0 { break; }  // EOF

        let line = buf.strip_suffix(b"\n").unwrap_or(&buf);

        let mut rotation: i32 = 0;

        for &byte in &line[1..] {
            rotation = rotation * 10 + (byte - b'0') as i32;
        }

        match line[0] {
            // rem_euclid always returns a non-negative result, unlike %
            b'L' => {
                position = (position - rotation).rem_euclid(100);
            },
            b'R' => {
                position = (position + rotation).rem_euclid(100);
            },
            _ => return Err(anyhow!("Invalid input"))
        }

        if position == 0 {
            result += 1;
        }
    }

    Ok(result)
}

/// Find number of times the lock passes 0 while making forwards and reverse rotations
/// from 0 to 99.
fn part2(reader: &mut impl BufRead) -> Result<usize> {
    let mut result = 0;
    let mut position: i32 = 50;
    let mut buf = Vec::new();

    loop {
        // Continually clearing keeps this loop zero-allocation
        buf.clear();

        // Read the line
        let bytes_read = reader.read_until(b'\n', &mut buf)?;

        if bytes_read == 0 { break; }  // EOF

        let line = buf.strip_suffix(b"\n").unwrap_or(&buf);

        let mut rotation: i32 = 0;

        for &byte in &line[1..] {
            rotation = rotation * 10 + (byte - b'0') as i32;
        }

        let rotation_sign: i32 = match line[0] {
            // rem_euclid always returns a non-negative result, unlike %
            b'L' => {
                -1
            },
            b'R' => {
                1
            },
            _ => return Err(anyhow!("Invalid input"))
        };

        // Loop and count number of times the position crosses a boundary
        while rotation > 0 {
            if rotation > 99 {
                rotation -= 100;
                result += 1;
            } else {
                position += rotation * rotation_sign;
                rotation = 0;
            }
        }

        // Cap position back between 0 - 99
        if position < 0 || position > 99 {
            position = position.rem_euclid(100);
            result += 1
        }

        if position == 0 {
            result += 1;
        }
    }

    Ok(result)
}

// --- Helpers ---

fn reader_from_file_path(path: &str) -> Result<BufReader<File>> {
    let file = File::open(path).with_context(|| format!("Failed to open file at path {}", path))?;
    Ok(BufReader::new(file))
}

fn read_lines(path: &str) -> Result<Vec<String>> {
    let file = File::open(path).with_context(|| format!("Failed to open {}", path))?;
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<_, _>>().map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &[u8] = b"L80\nR20";

    #[test]
    fn test_part1() {
        let mut reader = BufReader::new(TEST_INPUT);
        assert_eq!(part1(&mut reader).unwrap(), 0);
    }

    #[rstest]
    #[case(b"L30", 0)]
    #[case(b"R50", 1)]
    #[case(b"R50\nL50\nR50", 2)]
    #[case(b"L80\nR1000\nL30", 12)]
    fn test_part2(
        #[case] input: &[u8],
        #[case] expected: usize
    ) {
        let mut reader = BufReader::new(input);
        assert_eq!(part2(&mut reader).unwrap(), expected);
    }
}
