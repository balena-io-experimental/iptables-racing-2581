use std::process::Command;

use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Starting...");

    let supervisor_address = env::var("BALENA_SUPERVISOR_ADDRESS").unwrap();
    let supervisor_api_key = env::var("BALENA_SUPERVISOR_API_KEY").unwrap();

    loop {
        let url = format!("{}/ping", supervisor_address);

        let output = Command::new("curl")
            .arg("-X")
            .arg("GET")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg(url)
            .output()
            .expect("failed to execute process");

        let stdout = std::str::from_utf8(&output.stdout).unwrap();

        println!("{}", stdout);

        if stdout.find("OK").is_some() {
            println!("Supervisor responded to ping");
            break;
        }

        println!("Awaiting supervisor...");

        sleep(Duration::from_secs(5));
    }

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

    let mut first = None;
    let mut last = None;
    let mut detected = Vec::new();
    for (i, line) in stdout.lines().enumerate() {
        if line.find("iptables").is_some() {
            if first.is_none() {
                first = Some(i);
            } else {
                last = Some(i);
            }

            detected.push(line);
        }
    }

    let count = if first.is_some() || last.is_some() {
        last.unwrap() - first.unwrap() + 1
    } else {
        println!("First or Last not set!");
        0
    };

    println!("iptables count {}", count);

    if count == 15 {
        sleep(Duration::from_secs(20));

        println!("Rebooting...");

        let url = format!("{}/v1/reboot?apikey={}", supervisor_address, supervisor_api_key);

        let _ = Command::new("curl")
                .arg("-X")
                .arg("POST")
                .arg("-H")
                .arg("Content-Type: application/json")
                .arg(url)
                .output()
                .expect("failed to execute process");

        sleep(Duration::from_secs(3 * 60));
    } else {
        for line in detected {
            println!("{}", line);
        }
    
        println!("iptables count does not match");

        sleep(Duration::from_secs(60 * 60 * 12));
    }
}
