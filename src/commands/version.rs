pub(crate) fn version() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("git-timer {}", VERSION);
}
