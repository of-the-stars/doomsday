use doomsday::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let number = 12;
    let month = Month::get_month_from_number(number).ok_or(Error::last_os_error())?;

    println!("{} is month number {}", month, number);

    Ok(())
}
