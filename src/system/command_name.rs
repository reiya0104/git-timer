use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum CommandName {
    START,
    STOP,
    SHOW,
    HELP,
    VERSION,
}

impl FromStr for CommandName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(CommandName::START),
            "stop" => Ok(CommandName::STOP),
            "show" => Ok(CommandName::SHOW),
            "help" => Ok(CommandName::HELP),
            "--help" => Ok(CommandName::HELP),
            "-h" => Ok(CommandName::HELP),
            "version" => Ok(CommandName::VERSION),
            "--version" => Ok(CommandName::VERSION),
            "-v" => Ok(CommandName::VERSION),
            _ => Err(()),
        }
    }
}
