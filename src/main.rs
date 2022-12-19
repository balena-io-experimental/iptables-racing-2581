use std::process::Command;

use std::env;

fn main() {
    println!("Starting...");

    let supervisor_address = env::var("BALENA_SUPERVISOR_ADDRESS").unwrap();
    let supervisor_api_key = env::var("BALENA_SUPERVISOR_API_KEY").unwrap();

    let url = format!("{}/v2/journal-logs?apikey={}", supervisor_address, supervisor_api_key);

    let output = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("--data")
            .arg("{\"unit\":\"NetworkManager\",\"all\":true}")
            .arg(url)
            .output()
            .expect("failed to execute process");

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    let mut count = 0;
    for line in stdout.lines() {
        if line.find("iptables").is_some() {
            println!("{}", line);
            count += 1;
        }
    }

    println!("iptables count {}", count);

    std::thread::sleep(std::time::Duration::from_secs(60 * 60));
}
