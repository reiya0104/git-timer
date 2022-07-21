pub mod error;
pub mod git_timer;
pub mod system;

use anyhow;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    match git_timer::git_timer(args) {
        Ok(()) => Ok(()),
        Err(e) => Err(e),
    }
}
