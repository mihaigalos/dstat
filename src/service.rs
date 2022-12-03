pub fn systemctl_is_active(service: &str) -> bool {
    use std::process::Command;
    let output = Command::new("systemctl")
        .args(["is-active", service])
        .output()
        .expect(&format!("Unknown service {}.", service));

    if output.status.code().unwrap() == 0 {
        return true;
    }
    false
}
