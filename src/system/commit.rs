use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

pub(crate) fn commit(prefix: String, message: String) {
    let mut child = Command::new("git")
        .args(&[
            "commit",
            "--allow-empty",
            "-m",
            format!("{}{}", prefix, message).as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed");

    let stdout = child.stdout.take().unwrap();

    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{:?}", line);
    }
}

pub(crate) fn get_commit_message(args: Vec<String>) -> String {
    if args.len() >= 3 {
        return ": ".to_string() + args[2..].join(" ").as_str();
    }

    "".to_string()
}
