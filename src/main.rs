use std::env;
use dstat::colorizer::colorize;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    colorize(arguments, systemctl_is_active);
}

