extern crate serde_yaml;

extern crate serde;

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::{ffi::OsStr, process::Command};

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does build things
    Build {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_args: Vec<String>,
    },

    Deploy {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_args: Vec<String>,
    },
}

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

fn exec_command(program: &str, command: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(&program);
    exec_command.args(command);
    // exec_command.envs(envs);
    exec_command.env("BACKEND_TAG", "0.0.2");
    let output = exec_command.output().expect("failed to execute process");
    let args: Vec<&OsStr> = exec_command.get_args().collect();
    let envs: Vec<(&OsStr, Option<&OsStr>)> = exec_command.get_envs().collect();

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    println!(
        "Exec command: {:?} envs:{:?} args:{:?} {:?} {:?} {:?}",
        program, &envs, &args, status, stdout, stderr
    );
    Ok(stdout)
}

fn call_host(
    host: &RemoteHostCall,
    program: &str,
    command: Vec<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(&program);
    if host.private_key.is_some() {
        exec_command
            .arg("-i")
            .arg(host.private_key.as_ref().unwrap());
    }
    for arg in command {
        exec_command.arg(arg);
    }
    let output = exec_command.output().expect("failed to execute process");
    let args: Vec<&OsStr> = exec_command.get_args().collect();

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    println!(
        "Exec command: {:?} {:?} {:?} {:?} {:?}",
        program, &args, status, stdout, stderr
    );
    Ok(stdout)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Build { names, build_args }) => {
            let f = std::fs::File::open("tests/docker-compose.yaml").expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            println!("{:?}", format);

            let _ = build(&format, &names, build_args);
        }
        Some(Commands::Deploy { names, build_args }) => {
            println!("names: {:?}", names);
            println!("build_args: {:?}", build_args);

            let f = std::fs::File::open("tests/docker-compose.yaml").expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            println!("{:?}", format);

            let _x = deploy_test_backend(&format, names, build_args);
        }
        None => {}
    }
    Ok(())
}

fn build(
    format: &DockerComposeFormat,
    names: &Vec<String>,
    build_args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = Scope {
        hosts: vec!["root@46.101.98.131".to_string()],
        registry: "registry.digitalocean.com".to_string(),
        registry_auth_config: ".secrets/docker-config.json".to_string(),
        registry_export_auth_config: "/root/.docker/config.json".to_string(),
        path: "tests".to_string(),
    };

    // let envs: vec![
    //     (OsString::from("BACKEND_TAG"), Some(OsString::from("0.0.1"))),
    // ];
    let _ = exec_command(
        "docker",
        vec![
            "compose",
            "-f",
            &(scope.path.clone() + "/docker-compose.yaml"),
            "-f",
            "tests/docker-compose.override-run.prod.yaml",
            "build",
            "backend",
            "--push",
            "--build-arg",
            "BACKEND_TAG=0.0.1",
        ],
    );
    Ok(())
}

fn deploy_test_backend(
    format: &DockerComposeFormat,
    names: &Vec<String>,
    build_args: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let host = RemoteHostCall {
        private_key: Some("~/.ssh/dkrpublish_rsa".to_string()),
    };

    let scope = Scope {
        hosts: vec!["root@46.101.98.131".to_string()],
        registry: "registry.digitalocean.com".to_string(),
        registry_auth_config: ".secrets/docker-config.json".to_string(),
        registry_export_auth_config: "/root/.docker/config.json".to_string(),
        path: "tests".to_string(),
    };

    let run_file = std::fs::File::create("tests/.dkr-generated/docker-compose.override-run.yaml")
        .expect("Could not open file.");
    let mut run_format = DockerComposeFormat {
        version: format.version.clone(),
        services: Mapping::new(),
        volumes: Mapping::new(),
    };
    let mut run_service_map = Mapping::new();
    let mut run_environment_map = Mapping::new();
    // override environment
    // run_environment_map.insert(Value::String((&"xxx").to_string()), "yyy".into());
    // run_environment_map.insert(Value::String((&"DATABASE_URL").to_string()), "zzz".into());
    run_service_map.insert(Value::String((&"build").to_string()), "!reset null".into());
    run_service_map.insert(
        Value::String((&"environment").to_string()),
        Value::Mapping(run_environment_map),
    );

    run_format.services.insert(
        Value::String((&"backend").to_string()),
        Value::Mapping(run_service_map),
    );
    serde_yaml::to_writer(run_file, &run_format).expect("Could not write values.");

    let host0 = &scope.hosts[0];
    let host0_path = scope.hosts[0].clone() + ":.";

    let _ = call_host(
        &host,
        &"scp",
        vec![
            &scope.registry_auth_config,
            &(host0.clone() + ":" + &scope.registry_export_auth_config),
        ],
    )
    .expect("Failed to call host.");
    let _ = call_host(
        &host,
        &"ssh",
        vec![host0, &("docker login ".to_owned() + &scope.registry)],
    )
    .expect("Failed to call host.");

    let _ = call_host(&host, &"scp", vec!["-r", &scope.path, &host0_path])
        .expect("Failed to call host.");

    let mut deploy_command = "BACKEND_TAG=0.0.1".to_owned();
    deploy_command += " docker compose -f ";
    deploy_command += &(scope.path.clone() + "/docker-compose.yaml");
    deploy_command += " -f tests/docker-compose.override-run.prod.yaml";
    deploy_command += " -f tests/.dkr-generated/docker-compose.override-run.yaml";
    deploy_command += " up -d";

    let _x = call_host(&host, &"ssh", vec![host0, &deploy_command]).expect("Failed to call host.");

    Ok(())
}
