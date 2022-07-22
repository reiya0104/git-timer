use std::str::FromStr;

use crate::commands::version::version;
use crate::error::git_timer_error::GitTimerError;
use crate::system::{command_name::CommandName, start::start, stop::stop};

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
        Ok(CommandName::START) => Ok(start(args)),
        Ok(CommandName::STOP) => Ok(stop(args)),
        Ok(CommandName::HELP) => Ok(()),
        Ok(CommandName::VERSION) => Ok(version()),
        _ => Err(GitTimerError::InvalidCommandError {
            command: command.clone(),
        }
        .into()),
    }
}
