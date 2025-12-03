pub fn start_day(day: &str) {
    println!("Advent of Code 2025 - Day {:0>2}", day);
}

// Additional common functions

/// Much faster than parse
///
/// # Arguments
/// * slice: The bytes to parse. MUST only contain the digits of the number
/// 
/// # Returns
/// Pair where first value is the actual number, and second value is the length of the number
pub fn parse_int_ascii(slice: &[u8]) -> (usize, usize) {
    let mut val = 0;
    let mut len = 0;

    while len < slice.len() {
        val *= 10;  // Shift decimal space
        val += (slice[len] - b'0') as usize;  // 
        len += 1;
    }

    (val, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
