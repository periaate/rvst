use std::io::{self, IsTerminal};

/// Get all input args, including piped_args ones. Always skips the first item (the binary name).
///
/// ```ignore
/// // if ran with: `printf "Cargo.toml\nsrc" | cargo run -- "abc" 123`
/// let args: Vec<_> = all_args().collect();
/// assert_eq!(args, vec!["abc", "123", "Cargo.toml", "src"]);
/// ```
pub fn all_args() -> impl Iterator<Item = String> {
    std::env::args().skip(1).chain(piped_args().into_iter().flatten().filter_map(Result::ok))
}

/// Identical to `all_args` but doesn't ignore ignoring piped_args input errors.
pub fn all_args_raw() -> impl Iterator<Item = io::Result<String>> {
    std::env::args().skip(1).map(Ok).chain(piped_args().into_iter().flatten())
}

/// Get piped_args stdin args, one per line. Returns `None` if stdin is a terminal.
///
/// ```ignore
/// // if ran with: `printf "Cargo.toml\nsrc" | cargo run -- "abc" 123`
/// let piped_args: Vec<_> = piped_args().unwrap().map(Result::unwrap).collect();
/// assert_eq!(piped_args, vec!["Cargo.toml", "src"]);
/// ```
pub fn piped_args() -> Option<impl Iterator<Item = io::Result<String>>> {
    let stdin = io::stdin();
    if !stdin.is_terminal() { return None }
    Some(stdin.lines())
}
