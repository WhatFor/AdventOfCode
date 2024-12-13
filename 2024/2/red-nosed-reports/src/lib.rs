use std::{fs::File, io::{BufRead, BufReader}};
use anyhow::Result;

#[derive(PartialEq)]
enum Direction { Unknown, Rising, Falling }

pub fn count_safe_reports() -> Result<u16> {
    let input = read_input("./input")?;
    let mut count = 0;

    for i in input {
        let mut safe = true;
        let mut dampened = false;
        let mut dampened_index = 0;
        let mut direction = Direction::Unknown;

        for (index, current) in i.iter().enumerate() {
            if index + 1 == i.len() {
                // We're at the end
                break;
            }

            // TODO: this doesn't work
            if index == dampened_index {
                // Skip
                continue;
            }

            let next = i[index + 1];

            let diff = current - next;
            let abs = diff.abs();

            if !(1..4).contains(&abs) {
                // We have moved too little or too much
                if dampened {
                    safe = false;
                    break;
                } else {
                    dampened = true;
                    dampened_index = index;

                    continue;
                }
            }

            let current_direction = match diff {
                x if x > 0 => Direction::Falling,
                x if x < 0 => Direction::Rising,
                _ => Direction::Unknown,
            };

            if direction == Direction::Unknown {
                // We haven't observed a direction yet
                direction = current_direction;
            } else {
                // We have a direction... are we on path?
                if current_direction != direction {
                    if dampened {
                        safe = false;
                        break;
                    } else {
                        dampened = true;
                        dampened_index = index;
                        continue;
                    }
                }
            }
        }

        if safe {
            count += 1;
        }
    }

    Ok(count)
}

fn read_input(path: &str) -> Result<Vec<Vec<i8>>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::with_capacity(24);

    let mut reports = Vec::with_capacity(1000);

    while let Ok(len) = reader.read_line(&mut buffer) {
        if len == 0 {
             break;
        }

        let split = buffer.split_whitespace();
        let mut vec = vec![];

        for s in split {
            let num = s.parse::<i8>()?;
            vec.push(num);
        }

        reports.push(vec);
        buffer.clear();
    }

    Ok(reports)
}