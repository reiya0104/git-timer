use std::str::FromStr;

use crate::system::{command_name::CommandName, start::start, stop::stop};

pub(crate) fn git_timer(args: Vec<String>) {
    if args.len() <= 1 {
        println!("No args");
        return;
    }

    let command = &args[1];

    match CommandName::from_str(command) {
        Ok(CommandName::START) => start(args),
        Ok(CommandName::STOP) => stop(args),
        _ => println!("Invalid args"),
    }
}
