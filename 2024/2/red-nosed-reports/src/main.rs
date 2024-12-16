use anyhow::Result;
use reports::count_safe_reports;

fn main() -> Result<()> {
    let safe_reports_count = count_safe_reports()?;

    if safe_reports_count == 536 {
        println!("Safe reports: {}", safe_reports_count);
    } else {
        panic!("Wrong answer! {} is not correct. Expected 536.", safe_reports_count);
    }

    Ok(())
}
