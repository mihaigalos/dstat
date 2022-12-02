use colored::*;

fn systemctl_is_active(_service: &str) -> bool {
    false
}

pub fn colorize(arguments: Vec<String>) {
    if arguments.len() == 1 {
        println!("Usage: {} <list of units to inspect>",env!("CARGO_PKG_NAME"));
    }

    for service in arguments.iter().skip(1) {
        let bullet = ColoredString::from("●");
        let bullet = match systemctl_is_active(service){
            true => bullet.green(),
            false => bullet.red(),
        };

        print!("{} {} ", bullet, service);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_works_when_typical() {
        let result: Vec<ColoredString> = colorize(input, "(.*) (.*)", "cyan magenta").unwrap();

        assert_eq!(result[0], expected1);
        assert_eq!(result[1], expected2);
    }
}
