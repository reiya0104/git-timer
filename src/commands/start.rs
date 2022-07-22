use crate::system::commit::{commit, get_commit_message};

pub(crate) fn start(args: Vec<String>) {
    let message: String = get_commit_message(args);
    let prefix: String = "start".to_string();
    commit(prefix, message);
}
