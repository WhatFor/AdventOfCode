use historian::{find_distance, find_similarity};

fn main() {
    divan::main();
}

#[divan::bench()]
fn part_1() {
    let _ = find_distance();
}

#[divan::bench()]
fn part_2() {
    let _ = find_similarity();
}