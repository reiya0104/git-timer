use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum CommandName {
    START,
    STOP,
}

impl FromStr for CommandName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(CommandName::START),
            "stop" => Ok(CommandName::STOP),
            _ => Err(()),
        }
    }
}
