extern crate serde_yaml;

extern crate serde;

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::process::Command;
// use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct DockerComposeFormat {
    version: String,
    services: Mapping,
    volumes: Mapping,
    // city: String,
    // nums: Vec<Foo>,
}

struct HostCall {
    program: String,
    private_key: Option<String>,
    name_and_host: String,
}

fn call_host(host: &HostCall, command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = std::process::Command::new(&host.program);
    if host.private_key.is_some() {
        exec_command.arg("-i").arg(host.private_key.as_ref().unwrap());
    }
    exec_command.arg(&host.name_and_host).arg(command); // Replace 'command' with '&command'
    let output = exec_command.output().expect("failed to execute process");
    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    println!("From command: {:?} {:?} {:?} {:?}", command, status, stdout, stderr);
    Ok(stdout)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("tests/docker-compose.test.yaml").expect("Could not open file.");
    let format: DockerComposeFormat = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", format);

    let x = format.services["backend"]["build"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("Could not find key foo.bar in something.yaml");

    println!("Item {:?}", x?);

    let host = HostCall {
        program: "ssh".to_string(),
        private_key: Some("~/.ssh/dkrpublish_rsa".to_string()),
        name_and_host: "root@46.101.98.131".to_string(),
    };

    let x = call_host(&host, "docker run hello-world").expect("Failed to call host.");

    Ok(())
}
