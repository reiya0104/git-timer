use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum CommandName {
    START,
    STOP,
    HELP,
    VERSION,
}

impl FromStr for CommandName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(CommandName::START),
            "stop" => Ok(CommandName::STOP),
            "--help" => Ok(CommandName::HELP),
            "--version" => Ok(CommandName::VERSION),
            _ => Err(()),
        }
    }
}
