use anyhow::Result;
use mull_it_over::{get_total_with_do_and_donts};

fn main() -> Result<()> {
    let result = get_total_with_do_and_donts();

    println!("Result: {:?}", result);

    Ok(())
}