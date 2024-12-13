use historian::find_distance;

fn main() {
    divan::main();
}

#[divan::bench()]
fn historian_hysteria() {
    let _ = find_distance();
}