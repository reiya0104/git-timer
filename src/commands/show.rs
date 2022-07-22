use crate::system::git_log::git_log;

pub(crate) fn show(_args: Vec<String>) {
    let pretty = "format:\"[%ad] %h %an : %s\"";
    let greps = vec!["start", "stop"];

    git_log(pretty.to_string(), greps);
}
