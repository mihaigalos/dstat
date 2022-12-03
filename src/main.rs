use dstat::colorizer::colorize;
use dstat::service::is_active;
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let statuses = colorize(arguments, is_active);

    for status in statuses {
        print!("{} ", status);
    }
    println!("");
}
