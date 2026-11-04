use rand::prelude::*;

use std::{
    fmt,
    io::{self, prelude::*},
    str, time,
};

use crate::{
    date_logic::{DateStamp, DayOfTheWeek},
    helper_types::{CalError, GuessTime, Result},
};

#[derive(Debug)]
/// Represents the cli options available to configure the game
pub(crate) enum CliOption {
    /// The lower limit, inclusive, for the years genereated
    LowerYearLimit,
    /// The upper limit, inclusive, for the years genereated
    UpperYearLimit,
    /// The number of times the loop goes through
    LoopLimit,
    /// Whether or not to print the configuration at the start of the game
    ShowConfig,
    /// A wrapper around [`u16`] values for the limit variants
    UIntValue(u16),
}

impl str::FromStr for CliOption {
    type Err = CalError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "--show-config" => Ok(Self::ShowConfig),
            "--loop-limit" => Ok(Self::LoopLimit),
            "--upper-year-limit" => Ok(Self::UpperYearLimit),
            "--lower-year-limit" => Ok(Self::LowerYearLimit),
            value if let Ok(value) = value.trim().parse().map_err(|e| CalError::IntError(e)) => {
                Ok(CliOption::UIntValue(value))
            }
            _ => Err(CalError::ArgError),
        }
    }
}

#[derive(Debug)]
pub struct GameConfig {
    loop_limit: Option<u16>,
    show_config: bool,
    upper_year_limit: u16,
    lower_year_limit: u16,
}

impl GameConfig {
    /// Takes in a reference to a collection of [`String`]s, meant to be std::env::args() collected
    /// into a [`Vec<String>`], and parses them into a [`GameConfig`] configuration struct
    pub fn new(args: &[String]) -> Self {
        let mut config = Self {
            loop_limit: None,
            show_config: false,
            lower_year_limit: 1583,
            upper_year_limit: 2500,
        };

        let args: Vec<CliOption> = args
            .iter()
            .filter_map(|s| s.as_str().parse().ok())
            .collect();

        let next_item = |index| {
            if let Some(CliOption::UIntValue(value)) = args.get(index + 1) {
                Some(value)
            } else {
                None
            }
        };

        for (i, item) in args.iter().enumerate() {
            let next_value = next_item(i);

            match item {
                CliOption::LowerYearLimit if let Some(value) = next_value => {
                    config.lower_year_limit = *value;
                }
                CliOption::UpperYearLimit if let Some(value) = next_value => {
                    config.upper_year_limit = *value;
                }
                CliOption::LoopLimit if let Some(value) = next_value => {
                    config.loop_limit = Some(*value);
                }
                CliOption::ShowConfig => config.show_config = true,
                _ => {}
            }
        }

        config
    }
}

impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loop_limit_representation = if let Some(limit) = self.loop_limit {
            format!("Running loop {} time(s)", limit)
        } else {
            format!("No loop limit")
        };

        write!(
            f,
            "[ Running with the following configuration ] \n\n\
                - Upper year limit: {} \n\
                - Lower year limit: {} \n\
                - {} \n\n\
            [ ---------------------------------------- ]",
            self.upper_year_limit, self.lower_year_limit, loop_limit_representation
        )
    }
}

pub fn conway_login_game(config: GameConfig) -> Result<()> {
    if config.show_config {
        println!("{}\n", config);
    }

    let Some(counter) = config.loop_limit else {
        loop {
            guess_doomsday(&config)?;
            println!("");
        }
    };

    for _ in 1..=counter {
        guess_doomsday(&config)?;
        println!("");
    }

    Ok(())
}

fn guess_doomsday(config: &GameConfig) -> Result<()> {
    let lower_year_limit = config.lower_year_limit;
    let upper_year_limit = config.upper_year_limit;

    let mut date: Result<DateStamp> = Err(CalError::InvalidDate);

    while let Err(_) = date {
        let mut rng = rand::rng();
        let day = rng.random_range(1..=31);
        let month = rng.random_range(1..=12);
        let year = rng.random_range(lower_year_limit..=upper_year_limit);

        let tuple = (day, month, year);
        date = DateStamp::try_from_tuple(tuple);
    }

    let date = date?;

    println!("What day of the week is {}?\n", date);

    let day_of_the_week = date.get_day_of_the_week()?;

    let mut guess_buffer = String::new();
    let mut guess: Result<DayOfTheWeek>;

    let start = time::Instant::now();

    loop {
        print!("Guess: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut guess_buffer)?;
        guess = guess_buffer.trim().parse();
        guess_buffer.clear();

        if let Ok(day) = guess
            && day_of_the_week == day
        {
            println!("\nThat's correct!!\n");
            break;
        }
    }

    let end = time::Instant::now();

    println!("{} is a {}", date, day_of_the_week);

    let guess_time = GuessTime::new(end - start);
    println!("That took you {}", guess_time);

    Ok(())
}
