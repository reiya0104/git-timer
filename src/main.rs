pub mod git_timer;
pub mod system;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    git_timer::git_timer(args);
}
