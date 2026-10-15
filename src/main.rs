use doomsday::{self, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let limit: Option<u8> = if let Some(number_str) = args.get(1) {
        Some(number_str.trim().parse()?)
    } else {
        None
    };

    doomsday::conway_login_game(limit)?;
    Ok(())
}
