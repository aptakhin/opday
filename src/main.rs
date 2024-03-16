extern crate serde_yaml;

extern crate serde;

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};
use std::{ffi::OsStr, process::Command};

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::{debug, error, info, trace, warn};
use toml::Table;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    ssh_private_key: Option<PathBuf>,

    /// Verbose level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

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
        build_arg: Vec<String>,
    },

    Deploy {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
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
    docker_compose_overrides: Vec<String>,
    ssh_private_key: Option<String>,
}

struct Configuration {
    path: String,
    // current_scope: Scope,
    environments: Vec<Scope>,
}

fn exec_command(
    program: &str,
    command: Vec<&str>,
    build_arg: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(&program);
    exec_command.args(command);
    for build_arg_item in build_arg {
        let parts: Vec<&str> = build_arg_item.splitn(2, '=').collect();
        if parts.len() < 2 {
            panic!("Invalid build-arg without `=`: `{}`", build_arg_item)
        }
        exec_command.env(parts[0], parts[1]);
    }
    let output = exec_command.output().expect("failed to execute process");
    let args: Vec<&OsStr> = exec_command.get_args().collect();
    let envs: Vec<(&OsStr, Option<&OsStr>)> = exec_command.get_envs().collect();

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    debug!(
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
    debug!(
        "Exec remote command: {:?} {:?} {:?} {:?} {:?}",
        program, &args, status, stdout, stderr
    );
    Ok(stdout)
}

fn get_string_value<'a>(current: &'a Table, base: &'a Table, key: &str) -> Option<String> {
    if current.contains_key(key) {
        return Some(current[key].as_str().unwrap().to_string());
    } else if base.contains_key(key) {
        return Some(base[key].as_str().unwrap().to_string());
    }
    None
}

fn get_string_array_value<'a>(
    current: &'a Table,
    base: &'a Table,
    key: &str,
) -> Option<Vec<String>> {
    if current.contains_key(key) {
        return Some(
            current[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
        );
    } else if base.contains_key(key) {
        return Some(
            base[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
        );
    }
    None
}

fn push_parsing_scope(current: &Table, base: &Table) -> Scope {
    let registry = get_string_value(current, base, "registry");
    let hosts = get_string_array_value(current, base, "hosts");
    let registry_auth_config = get_string_value(current, base, "registry_auth_config");
    let registry_export_auth_config =
        get_string_value(current, base, "registry_export_auth_config");
    let docker_compose_overrides =
        get_string_array_value(current, base, "docker_compose_overrides");
    let ssh_private_key = get_string_value(current, base, "ssh_private_key");

    return Scope {
        hosts: hosts.unwrap(),
        registry: registry.unwrap(),
        registry_auth_config: registry_auth_config.unwrap(),
        registry_export_auth_config: registry_export_auth_config.unwrap(),
        docker_compose_overrides: docker_compose_overrides.unwrap(),
        ssh_private_key: ssh_private_key,
    };
}

fn read_configuration(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(&path);
    let file = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let cfg: Table = file.parse().unwrap();

    let mut environments: Vec<Scope> = vec![];
    let mut base_scope = Table::new();

    let val: Table = cfg["environments"].as_table().unwrap().clone();
    for (key, value) in val.iter() {
        if value.is_table() {
            continue;
        }
        debug!("Looking into key: {:?}; Value: {:?}", key, value);
        base_scope.insert(key.clone(), value.clone());
    }

    for (key, value) in val.iter() {
        if !value.is_table() {
            continue;
        }
        debug!("Filling into environment: {:?}", key);

        let scope = push_parsing_scope(&value.as_table().unwrap(), &base_scope);
        environments.push(scope);
    }

    let config: Configuration = Configuration {
        path: cfg["path"].as_str().unwrap().to_string(),
        environments: environments,
    };
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    env_logger::init();

    let config =
        read_configuration("tests/dkrdeliver.test.toml").expect("Could not read configuration.");

    match &cli.command {
        Some(Commands::Build { names, build_arg }) => {
            let f = std::fs::File::open("tests/docker-compose.yaml").expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _ = build(&config, &format, &names, build_arg);
        }
        Some(Commands::Deploy { names, build_arg }) => {
            let f = std::fs::File::open("tests/docker-compose.yaml").expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _x = deploy(&config, &format, names, build_arg);
        }
        None => {}
    }
    Ok(())
}

fn build(
    config: &Configuration,
    format: &DockerComposeFormat,
    names: &Vec<String>,
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = exec_command(
        "docker",
        vec![
            "compose",
            "-f",
            &(config.path.clone() + "/docker-compose.yaml"),
            "-f",
            "tests/docker-compose.override-run.prod.yaml",
            "build",
            "backend",
            "--push",
        ],
        build_arg,
    );
    Ok(())
}

fn deploy(
    config: &Configuration,
    format: &DockerComposeFormat,
    names: &Vec<String>,
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = &config.environments[0];
    let host = RemoteHostCall {
        private_key: scope.ssh_private_key.clone(),
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

    let _ = call_host(&host, &"scp", vec!["-r", &config.path, &host0_path])
        .expect("Failed to call host.");

    let mut deploy_command = String::new();
    for build_arg_item in build_arg {
        deploy_command += build_arg_item;
    }
    deploy_command += " docker compose -f ";
    deploy_command += &(config.path.clone() + "/docker-compose.yaml");
    for override_file in &scope.docker_compose_overrides {
        deploy_command += " -f ";
        deploy_command += &override_file;
    }
    deploy_command += " -f tests/.dkr-generated/docker-compose.override-run.yaml";
    deploy_command += " up -d";

    let _x = call_host(&host, &"ssh", vec![host0, &deploy_command]).expect("Failed to call host.");

    Ok(())
}
