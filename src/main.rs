use std::env;

fn main() -> doomsday::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = doomsday::GameConfig::new(&args)?;

    doomsday::conway_login_game(config)?;

    Ok(())
}
