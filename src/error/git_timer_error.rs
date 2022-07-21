use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitTimerError {
    #[error("argument {:?}, this length is {}: Invalid argument error. At least {} aruguments are required.", .argument, .length, .min_length)]
    InvalidArgumentError {
        argument: Vec<String>,
        length: usize,
        min_length: usize,
    },

    #[error("command {}: Invalid command error.", .command)]
    InvalidCommandError { command: String },
}
