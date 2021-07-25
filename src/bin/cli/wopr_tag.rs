extern crate wopr_tag;
use clap::{App, Arg};
use log::{debug, error};
use std::env;
use std::str::FromStr;

fn main() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_owned());
    // Default to info, throw exception if LOG_LEVEL is set but not valid.
    simple_logger::init_with_level(log::Level::from_str(&log_level).expect("Invalid log level specificed")).unwrap();
    process_command_line();
}

fn process_command_line() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Agent based simulation of the game tag")
        .arg(
            Arg::with_name("number_of_players")
                .value_name("number_of_players")
                .help("The number of players (agents) to participate in the game of tag.")
                .short("p")
                .long("num-players")
                .required(true)
                .takes_value(true)
                .validator(validate_num_players)
        )
        .arg(
            Arg::with_name("x_size")
                .value_name("x_size")
                .help("How long should the field of play be along the x-axis.")
                .short("x")
                .long("x-size")
                .required(true)
                .takes_value(true)
                .validator(validate_axis)
        )
        .arg(
            Arg::with_name("y_size")
                .value_name("y_size")
                .help("How long should the field of play be along the y-axis.")
                .short("y")
                .long("y-size")
                .required(true)
                .takes_value(true)
                .validator(validate_axis)
        )
        .arg(
            Arg::with_name("show_field")
                .value_name("show_field")
                .help("Should we display the field during simulation (true/false)")
                .short("s")
                .long("show-field")
                .required(false)
                .takes_value(true)
                .default_value("true")
                .validator(validate_bool)
        )
        .arg(
            Arg::with_name("wait_between_turn")
                .value_name("wait_between_turn")
                .help(
                    "How long to wait between turns in milliseconds. A lower values simulates faster. A higher value \
                     make it easier for a human to review output."
                )
                .short("w")
                .long("wait-between-turn")
                .required(false)
                .takes_value(true)
                .default_value("250")
                .validator(validate_wait)
        )
        .arg(
            Arg::with_name("num_turns")
                .value_name("num_turns")
                .help("how many turns players get to take")
                .short("t")
                .long("num-turns")
                .required(false)
                .takes_value(true)
                .default_value("1000")
                .validator(validate_num_turns)
        )
        .get_matches();

    // Unwrapping here is safe because we have already validated the inputs via Clap's
    // validation functionality.
    let num_players = matches.value_of("number_of_players").unwrap().parse::<usize>().unwrap();
    let x_size = matches.value_of("x_size").unwrap().parse::<usize>().unwrap();
    let y_size = matches.value_of("y_size").unwrap().parse::<usize>().unwrap();
    let wait = matches.value_of("wait_between_turn").unwrap().parse::<u64>().unwrap();
    let show_field = matches.value_of("show_field").unwrap().parse::<bool>().unwrap();
    let num_turns = matches.value_of("num_turns").unwrap().parse::<usize>().unwrap();

    debug!(
        "cli args - number_of_players: {}, x_size: {}, y_size: {}, wait: {}, show_field: {}, num_turns: {}",
        num_players, x_size, y_size, wait, show_field, num_turns
    );

    if x_size * y_size < num_players {
        error!(
            "{} players cannot fit on field of dimenstions {} x {}",
            num_players, x_size, y_size
        );
    } else {
        wopr_tag::init(num_players, x_size, y_size, wait, show_field, num_turns);
    }
}

fn validate_num_players(players: String) -> Result<(), String> {
    let players_parse_result = players.parse::<usize>();

    if let Ok(players) = players_parse_result {
        if players >= 3 {
            return Ok(());
        }
    };

    let err_msg = format!(
        "the number of players must be a valid integer between 3 - {} inclusive.",
        usize::MAX
    );

    Err(err_msg)
}

fn validate_axis(axis: String) -> Result<(), String> {
    let axis_parse_result = axis.parse::<usize>();

    if let Ok(axis) = axis_parse_result {
        if axis >= 3 {
            return Ok(());
        }
    };

    let err_msg = format!(
        "the axis length must be a valid integer between 3 - {} inclusive.",
        usize::MAX
    );

    Err(err_msg)
}

fn validate_wait(wait: String) -> Result<(), String> {
    if wait.parse::<u64>().is_ok() {
        return Ok(());
    }

    let err_msg = format!(
        "the wait time in ms must be a valid integer between 0 - {} inclusive.",
        u64::MAX
    );

    Err(err_msg)
}

fn validate_bool(bool_str: String) -> Result<(), String> {
    if bool_str.parse::<bool>().is_ok() {
        return Ok(());
    }

    Err("Value must be true or false".to_owned())
}

fn validate_num_turns(turns: String) -> Result<(), String> {
    let turns_parse_result = turns.parse::<usize>();

    if let Ok(turns) = turns_parse_result {
        if turns >= 10 {
            return Ok(());
        }
    };

    let err_msg = format!(
        "the number of turns must be a valid integer between 10 - {} inclusive.",
        usize::MAX
    );

    Err(err_msg)
}
