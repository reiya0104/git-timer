use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

pub(crate) fn git_log(pretty: String, greps: Vec<&str>) {
    let pretty_opt = format!("--pretty={}", pretty);
    let mut greps_opt: Vec<String> = greps.iter().map(|&x| format!("--grep=\"{}\"", x)).collect();

    let mut command_args: Vec<String> =
        vec!["--no-pager".to_string(), "log".to_string(), pretty_opt];

    command_args.append(&mut greps_opt);

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
