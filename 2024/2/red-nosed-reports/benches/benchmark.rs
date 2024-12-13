use reports::count_safe_reports;

fn main() {
    divan::main();
}

#[divan::bench()]
fn part_1() {
    let _ = count_safe_reports();
}

#[divan::bench()]
fn part_2() {
}