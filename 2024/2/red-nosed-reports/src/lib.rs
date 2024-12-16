use std::{fs::File, io::{BufRead, BufReader}};
use anyhow::Result;

#[derive(PartialEq)]
enum Direction { Unknown, Rising, Falling }

pub fn count_safe_reports() -> Result<u16> {
    let input = read_input("./input")?;
    let mut count = 0;

    for row in input {
        let safety = is_safe(&row);

        if safety.is_ok() {
            count += 1;
        }
    }

    Ok(count)
}

pub fn count_safe_reports_with_dampener() -> Result<u16> {
    let input = read_input("./input")?;
    let mut count = 0;

    for row in input {
        let safety = is_safe(&row);

        match safety {
            Ok(_) => count += 1,
            Err(e) => {
                // let mut reattempt_row = row.clone();
                // reattempt_row.remove(e.unsafe_index + 1);
                // let reattempt = is_safe(&reattempt_row);

                // if reattempt.is_ok() {
                //     count += 1;
                // }

                // let mut final_reattempt_row = row.clone();
                // println!("Trying... {:?}", final_reattempt_row);
                // final_reattempt_row.remove(e.unsafe_index);
                // println!("Trimmed... {:?}", final_reattempt_row);
                // let final_reattempt = is_safe(&final_reattempt_row);
                // println!("Result... {:?}", final_reattempt.is_ok());

                // if final_reattempt.is_ok() {
                //     count += 1;
                // }

                for i in 0..row.len() {
                    let mut c = row.clone();
                    c.remove(i);
                    let is_safe = is_safe(&c);
                    
                    if is_safe.is_ok() { 
                        count+= 1;
                        break;
                    }
                }
            }
        }
    }

    Ok(count)
}

struct SafetyError { unsafe_index: usize }

fn is_safe(vec: &[i8]) -> Result<(), SafetyError> {
        let mut direction = Direction::Unknown;

        for (index, current) in vec.iter().enumerate() {
            if index + 1 == vec.len() {
                // We're at the end
                break;
            }

            let next = vec[index + 1];

            let diff = current - next;
            let abs = diff.abs();

            if !(1..4).contains(&abs) {
                // We have moved too little or too much
                return Err(SafetyError { unsafe_index: index })
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
                    return Err(SafetyError { unsafe_index: index })
                }
            }
        }

        Ok(())
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