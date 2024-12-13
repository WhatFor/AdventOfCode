use anyhow::Result;
use reports::count_safe_reports;

fn main() -> Result<()> {
    let safe_reports_count = count_safe_reports()?;

    println!("Safe reports: {}", safe_reports_count);

    Ok(())
}
