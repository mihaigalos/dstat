use std::env;
use dstat::colorizer::colorize;

fn systemctl_is_active(_service: &str) -> bool {
    false
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    colorize(arguments, systemctl_is_active);
}

