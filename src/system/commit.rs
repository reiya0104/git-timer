pub(crate) fn commit(message: String) {
    println!("{:?}", message);
}

pub(crate) fn get_commit_message(args: Vec<String>) -> String {
    if args.len() >= 3 {
        return ": ".to_string() + args[2..].join(" ").as_str();
    }

    "".to_string()
}
