use anyhow::Result;
use std::{fs::File, io::{BufReader, Read}};

#[derive(PartialEq)]
enum Mode {
    Searching,
    FirstDigit,
    SecondDigit,
}

pub fn get_total() -> Result<u32> {
    let file = File::open("./input")?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let chars: Vec<char> = content.chars().collect();

    let mut_arr = vec!['m', 'u', 'l', '('];

    let mut mode = Mode::Searching;
    let mut first_digit = 0;
    let mut pos = 0;
    let mut total: u32 = 0;

    loop {
        if pos >= chars.len() {
            // END
            break;
        }

        let current_char = chars[pos];

        match current_char {
            'm' => {
                // match 'mut('
                let potential_match_chars = &chars[pos..(pos + 4)];
                let mut_match = potential_match_chars == mut_arr;
                // Move past
                if mut_match {
                    if mode == Mode::Searching {
                        // We've found 1st digit start
                        mode = Mode::FirstDigit;
                    }

                    pos += 4; // mul(
                } else {
                    pos += 1;
                }
            },
            n if current_char.is_numeric() => {
                if mode == Mode::Searching {
                    // We've not hit 'mul(' yet, so skip
                    pos += 1;
                    continue;
                }

                let mut digits = vec![n];

                while digits.len() < 3 {
                    let next = chars[pos + digits.len()];
                    let is_next_numeric = next.is_numeric();
                    if is_next_numeric {
                        digits.push(next);
                    } else {
                        break;
                    }
                }

                // move beyond digits
                pos += digits.len();

                let digit_string = digits.iter().collect::<String>();
                let num = digit_string.parse::<u32>()?;

                if mode == Mode::FirstDigit {
                    first_digit = num;

                    if chars[pos] == ',' {
                        // skip the comma
                        pos += 1;
                        // move to next number
                        mode = Mode::SecondDigit;
                    } else {
                        // we've got an imposter! Not a true mul op
                        pos += 1;
                        // reset
                        mode = Mode::Searching;
                    }

                } else {
                    let second_digit = num;

                    if chars[pos] == ')' {
                        // handle result, mul op
                        let mul = first_digit * second_digit;
                        total += mul;
                        // skip closing bracket
                        pos += 1;
                    } else {
                        // Imposter! don't do any work
                        pos += 1;
                    }

                    // return to searching
                    mode = Mode::Searching;
                    // reset
                    first_digit = 0;
                }
            }
            _ => {
                mode = Mode::Searching;
                pos += 1;
            }
        }
    }

    Ok(total)
}

pub fn get_total_with_do_and_donts() -> Result<u32> {
    let file = File::open("./input")?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let chars: Vec<char> = content.chars().collect();

    let mut_arr = vec!['m', 'u', 'l', '('];
    let do_arr = vec!['d', 'o', '('];
    let dont_arr = vec!['d', 'o', 'n', '\'', 't', '('];

    let mut mode = Mode::Searching;
    let mut first_digit = 0;
    let mut pos = 0;
    let mut total: u32 = 0;

    let mut mut_enabled = true;

    loop {
        if pos >= chars.len() {
            // END
            break;
        }

        let current_char = chars[pos];

        match current_char {
            'm' => {
                // match 'mut('
                let potential_match_chars = &chars[pos..(pos + mut_arr.len())];
                let mut_match = potential_match_chars == mut_arr;
                // Move past
                if mut_match {
                    if mode == Mode::Searching {
                        // We've found 1st digit start
                        mode = Mode::FirstDigit;
                    }

                    pos += 4; // mul(
                } else {
                    pos += 1;
                }
            },
            'd' => {
                let do_match = chars[pos..(pos + do_arr.len())] == do_arr;
                if do_match {
                    mut_enabled = true;
                    pos += do_arr.len();
                }

                let dont_match = chars[pos..(pos + dont_arr.len())] == dont_arr;
                if dont_match {
                    mut_enabled = false;
                    pos += dont_arr.len();
                }
            },
            n if current_char.is_numeric() => {
                if mode == Mode::Searching {
                    // We've not hit 'mul(' yet, so skip
                    pos += 1;
                    continue;
                }

                if !mut_enabled {
                    // No point in parsing numbers
                    pos += 1;
                    continue;
                }

                let mut digits = vec![n];

                while digits.len() < 3 {
                    let next = chars[pos + digits.len()];
                    let is_next_numeric = next.is_numeric();
                    if is_next_numeric {
                        digits.push(next);
                    } else {
                        break;
                    }
                }

                // move beyond digits
                pos += digits.len();

                let digit_string = digits.iter().collect::<String>();
                let num = digit_string.parse::<u32>()?;

                if mode == Mode::FirstDigit {
                    first_digit = num;

                    if chars[pos] == ',' {
                        // skip the comma
                        pos += 1;
                        // move to next number
                        mode = Mode::SecondDigit;
                    } else {
                        // we've got an imposter! Not a true mul op
                        pos += 1;
                        // reset
                        mode = Mode::Searching;
                    }

                } else {
                    let second_digit = num;

                    if chars[pos] == ')' {
                        // handle result, mul op
                        let mul = first_digit * second_digit;
                        total += mul;
                        // skip closing bracket
                        pos += 1;
                    } else {
                        // Imposter! don't do any work
                        pos += 1;
                    }

                    // return to searching
                    mode = Mode::Searching;
                    // reset
                    first_digit = 0;
                }
            }
            _ => {
                mode = Mode::Searching;
                pos += 1;
            }
        }
    }

    Ok(total)
}