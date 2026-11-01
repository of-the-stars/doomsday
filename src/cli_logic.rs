use rand::prelude::*;

use std::{
    fmt,
    io::{self, prelude::*},
    time,
};

use crate::{
    date_logic::{DateStamp, DayOfTheWeek},
    helper_types::{CalError, GuessTime, Result},
};

#[derive(Debug)]
enum CliOption {
    LowerYearLimit,
    UpperYearLimit,
    LoopLimit,
    ShowConfig,
    UIntValue(u16),
}

impl TryFrom<&str> for CliOption {
    type Error = CalError;

    fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
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
    pub fn new(args: &[String]) -> Result<Self> {
        let mut config = Self {
            loop_limit: None,
            show_config: false,
            lower_year_limit: 1583,
            upper_year_limit: 2500,
        };

        let args: Vec<CliOption> = args
            .iter()
            .filter_map(|s| s.as_str().try_into().ok())
            .collect();

        let length = args.len();

        for (i, item) in args.iter().enumerate() {
            match item {
                CliOption::LowerYearLimit
                    if i + 1 < length
                        && let Some(CliOption::UIntValue(value)) = args.get(i + 1) =>
                {
                    config.lower_year_limit = *value;
                }
                CliOption::UpperYearLimit
                    if i + 1 < length
                        && let Some(CliOption::UIntValue(value)) = args.get(i + 1) =>
                {
                    config.upper_year_limit = *value;
                }
                CliOption::LoopLimit
                    if i + 1 < length
                        && let Some(CliOption::UIntValue(value)) = args.get(i + 1) =>
                {
                    config.loop_limit = Some(*value);
                }
                CliOption::ShowConfig => config.show_config = true,
                _ => {}
            }
        }

        Ok(config)
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
