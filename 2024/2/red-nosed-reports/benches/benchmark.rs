fn main() {
    divan::main();
}

#[divan::bench()]
fn part_1() {
    let _ = reports::count_safe_reports();
}

#[divan::bench()]
fn part_2() {
    let _ = reports::count_safe_reports_with_dampener();
}