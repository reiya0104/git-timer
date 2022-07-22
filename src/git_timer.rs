use std::str::FromStr;

use crate::commands::*;
use crate::error::git_timer_error::GitTimerError;
use crate::system::command_name::CommandName;

pub(crate) fn git_timer(args: Vec<String>) -> anyhow::Result<()> {
    if args.len() <= 1 {
        return Err(GitTimerError::InvalidArgumentError {
            argument: args.clone(),
            length: args.len(),
            min_length: 2,
        }
        .into());
    }

    let command = &args[1];

    match CommandName::from_str(command) {
        Ok(CommandName::START) => Ok(start::start(args)),
        Ok(CommandName::STOP) => Ok(stop::stop(args)),
        Ok(CommandName::SHOW) => Ok(show::show(args)),
        Ok(CommandName::HELP) => Ok(println!("help")),
        Ok(CommandName::VERSION) => Ok(version::version()),
        _ => Err(GitTimerError::InvalidCommandError {
            command: command.clone(),
        }
        .into()),
    }
}
