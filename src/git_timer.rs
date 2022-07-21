use std::str::FromStr;

use crate::system::{commands::Commands, start::start, stop::stop};

pub(crate) fn git_timer(args: Vec<String>) {
    if args.len() <= 1 {
        println!("No args");
        return;
    }

    let command = &args[1];

    match Commands::from_str(command) {
        Ok(Commands::START) => start(),
        Ok(Commands::STOP) => stop(),
        _ => println!("Invalid args"),
    }
}
