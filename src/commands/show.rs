use crate::system::git_log::git_log;

pub(crate) fn show(_args: Vec<String>) {
    let pretty = "format:\"[%ad] %h %an : %s\"";
    let grep = r#"^\(start\|stop\)\(:.\+\)\?$"#;
    git_log(pretty, grep);
}
