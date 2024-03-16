extern crate serde_yaml;

extern crate serde;

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::{ffi::OsStr, process::Command};
// use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct DockerComposeFormat {
    version: String,
    services: Mapping,
    volumes: Mapping,
}

struct RemoteHostCall {
    private_key: Option<String>,
}

struct Scope {
    hosts: Vec<String>,
    registry: String,
    registry_auth_config: String,
    registry_export_auth_config: String,
    path: String,
}

fn call_host(host: &RemoteHostCall, program: &str, command: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(&program);
    if host.private_key.is_some() {
        exec_command.arg("-i").arg(host.private_key.as_ref().unwrap());
    }
    for arg in command {
        exec_command.arg(arg);
    }
    let output = exec_command.output().expect("failed to execute process");
    let args: Vec<&OsStr> = exec_command.get_args().collect();

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    println!("Exec command: {:?} {:?} {:?} {:?} {:?}", program, &args, status, stdout, stderr);
    Ok(stdout)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("tests/docker-compose.yaml").expect("Could not open file.");
    let format: DockerComposeFormat = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", format);

    let x = format.services["backend"]["build"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("Could not find key.");

    println!("Item {:?}", x?);

    let _x = deploy_test_backend(&format);

    Ok(())
}

fn deploy_test_backend(format: &DockerComposeFormat) -> Result<(), Box<dyn std::error::Error>> {
    let host = RemoteHostCall {
        private_key: Some("~/.ssh/dkrpublish_rsa".to_string()),
    };

    let scope = Scope {
        hosts: vec!["root@46.101.98.131".to_string()],
        registry: "registry.digitalocean.com".to_string(),
        registry_auth_config: "docker-config.json".to_string(),
        registry_export_auth_config: "/root/.docker/config.json".to_string(),
        path: "tests".to_string(),
    };

    let build_file = std::fs::File::create("tests/.dkr-generated/docker-compose.override-build.yaml").expect("Could not open file.");
    let mut build_f = DockerComposeFormat {
        version: format.version.clone(),
        services: Mapping::new(),
        volumes: Mapping::new(),
    };
    let mut service_map = Mapping::new();
    service_map.insert(Value::String((&"image").to_string()), "registry.digitalocean.com/frlr/dkrpublish/test-backend:latest".into());
    build_f.services.insert(Value::String((&"backend").to_string()), Value::Mapping(service_map));
    serde_yaml::to_writer(build_file, &build_f).expect("Could not write values.");

    let run_file = std::fs::File::create("tests/.dkr-generated/docker-compose.override-run.yaml").expect("Could not open file.");
    let mut run_format = DockerComposeFormat {
        version: format.version.clone(),
        services: Mapping::new(),
        volumes: Mapping::new(),
    };
    let mut run_service_map = Mapping::new();
    run_service_map.insert(Value::String((&"build").to_string()), "!reset null".into());
    run_format.services.insert(Value::String((&"backend").to_string()), Value::Mapping(run_service_map));
    serde_yaml::to_writer(run_file, &run_format).expect("Could not write values.");

    let host0 = &scope.hosts[0];
    let host0_path = scope.hosts[0].clone() + ":.";

    let _x = call_host(&host, &"scp", vec![&scope.registry_auth_config, &(host0.clone() + ":" + &scope.registry_export_auth_config)]).expect("Failed to call host.");
    let _x = call_host(&host, &"ssh", vec![host0, &("docker login ".to_owned() + &scope.registry)]).expect("Failed to call host.");

    let _x = call_host(&host, &"scp", vec!["-r", &scope.path, &host0_path]).expect("Failed to call host.");

    let mut deploy_command = "BACKEND_TAG=latest".to_owned();
    deploy_command += " docker compose -f ";
    deploy_command += &(scope.path.clone() + "/docker-compose.yaml");
    deploy_command += " -f tests/.dkr-generated/docker-compose.override-build.yaml";
    deploy_command += " -f tests/docker-compose.override-run.prod.yaml";
    deploy_command += " -f tests/.dkr-generated/docker-compose.override-run.yaml";
    deploy_command += " up -d";

    let _x = call_host(&host, &"ssh", vec![host0, &deploy_command]).expect("Failed to call host.");

    Ok(())
}
