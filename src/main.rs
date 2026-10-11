use doomsday::{DateStamp, Result};

fn main() -> Result<()> {
    // let day = DateStamp::try_from_input()?;
    // let day_of_the_week = day.get_day_of_the_week()?;
    //
    // println!("\n{} is a {}", day, day_of_the_week);
    //

    let month = DateStamp::test()?;

    println!("{:?}", month);

    Ok(())
}
