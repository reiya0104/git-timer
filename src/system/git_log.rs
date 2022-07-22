use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

pub(crate) fn git_log(pretty: &str, grep: &str) {
    let pretty_opt = format!("--pretty={}", pretty);
    let grep_opt = format!("--grep={}", grep);

    let command_args: Vec<&str> = vec!["--no-pager", "log", &pretty_opt, &grep_opt];

    let mut child = Command::new("git")
        .args(command_args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed");

    let stdout = child.stdout.take().unwrap();

    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{:?}", line);
    }
}
