use colored::*;

fn systemctl_is_active(_service: &str) -> bool {
    false
}

pub fn colorize(arguments: Vec<String>) -> Vec<ColoredString> {
    let mut result: Vec<ColoredString> = vec![];
    if arguments.len() == 1 {
        println!("Usage: {} <list of units to inspect>",env!("CARGO_PKG_NAME"));
    }

    for service in arguments.iter().skip(1) {
        let bullet = ColoredString::from("●");
        let bullet = match systemctl_is_active(service){
            true => bullet.green(),
            false => bullet.red(),
        };

        let intermediate : &str = &format!("{} {}", bullet, service)[..];
        result.push(ColoredString::from(intermediate));
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_works_when_typical() {
        let input = vec!["program_name".to_string(), "docker".to_string()];
        let expected_string = format!("{} {}",ColoredString::from("●").red(), "docker");
        let expected = vec![ColoredString::from(&expected_string[..])];

        let result: Vec<ColoredString> = colorize(input);

        assert_eq!(result[0], *expected.get(0).unwrap());
    }
}
