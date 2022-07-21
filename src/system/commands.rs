use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Commands {
    START,
    STOP,
}

impl FromStr for Commands {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Commands::START),
            "stop" => Ok(Commands::STOP),
            _ => Err(()),
        }
    }
}
