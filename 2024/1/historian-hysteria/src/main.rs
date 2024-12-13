use anyhow::Result;
use historian::find_distance;

fn main() -> Result<()> {
    let distance = find_distance()?;

    println!("Total disatance: {}", &distance);

    Ok(())
}
