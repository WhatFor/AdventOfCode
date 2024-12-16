use mull_it_over::{get_total, get_total_with_do_and_donts};

fn main() {
    divan::main();
}

#[divan::bench()]
fn part_1() {
    let _ = get_total();
}

#[divan::bench()]
fn part_2() {
    let _ = get_total_with_do_and_donts();
}