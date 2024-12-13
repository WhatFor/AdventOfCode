use anyhow::Result;
use historian::{find_distance, find_similarity};

fn main() -> Result<()> {
    let distance = find_distance()?;

    println!("Total disatance: {}", &distance);

    let similarity = find_similarity()?;

    println!("Total similarity: {}", &similarity);

    Ok(())
}
