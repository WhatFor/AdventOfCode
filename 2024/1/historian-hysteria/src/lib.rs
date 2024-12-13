use std::io::{BufRead, BufReader};
use anyhow::Result;

pub fn find_distance() -> Result<u32> {
    let (left, right)= read_input("./input")?;
    let mut sum: u32 = 0;

    for index in 0..left.len() {
        sum += left[index].abs_diff(right[index]);
    }

    Ok(sum)
}

fn read_input(path: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::with_capacity(13);

    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    while let Ok(len) = reader.read_line(&mut buffer) {
        if len == 0 {
             break;
        }

        let slice_l = &buffer[0..5];
        let l = slice_l.parse::<u32>()?;
        left.push(l);

        let slice_r = &buffer[8..13];
        let r = slice_r.parse::<u32>()?;
        right.push(r);

        buffer.clear();
    }

    left.sort();
    right.sort();

    Ok((left, right))
}