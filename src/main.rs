mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

use behavior_hiding::cli_parser::parse_input;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    parse_input();
}
