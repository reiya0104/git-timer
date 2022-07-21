use super::commit::{commit, get_commit_message};

pub(crate) fn start(args: Vec<String>) {
    let message: String = get_commit_message(args);

    commit(message);
}
