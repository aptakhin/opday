use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::debug;

mod config;
mod exec;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    env_logger::init();

    let config = config::read_configuration("tests/dkrdeliver.test.toml")
        .expect("Could not read configuration.");

    // let scope = &config.environments[0];

    match &cli.command {
        Some(Commands::Build { names, build_arg }) => {
            let f = std::fs::File::open(&config.docker_compose_file).expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _ = build(&config, &format, &names, build_arg);
        }
        Some(Commands::Deploy { names, build_arg }) => {
            let f = std::fs::File::open(&config.docker_compose_file).expect("Could not open file.");
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
    config: &config::Configuration,
    _format: &DockerComposeFormat,
    _names: &Vec<String>,
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = exec::exec_command(
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
    config: &config::Configuration,
    format: &DockerComposeFormat,
    _names: &Vec<String>,
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = &config.environments[0];
    let host = exec::RemoteHostCall {
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
    // override environment
    // run_environment_map.insert(Value::String((&"xxx").to_string()), "yyy".into());
    // run_environment_map.insert(Value::String((&"DATABASE_URL").to_string()), "zzz".into());
    run_service_map.insert(Value::String((&"build").to_string()), "!reset null".into());
    run_service_map.insert(
        Value::String((&"environment").to_string()),
        Value::Mapping(Mapping::new()),
    );

    run_format.services.insert(
        Value::String((&"backend").to_string()),
        Value::Mapping(run_service_map),
    );
    serde_yaml::to_writer(run_file, &run_format).expect("Could not write values.");

    let host0 = &scope.hosts[0];
    let host0_path = scope.hosts[0].clone() + ":.";

    let _ = exec::call_host(
        &host,
        &"scp",
        vec![
            &scope.registry_auth_config,
            &(host0.clone() + ":" + &scope.registry_export_auth_config),
        ],
    )
    .expect("Failed to call host.");
    let _ = exec::call_host(
        &host,
        &"ssh",
        vec![host0, &("docker login ".to_owned() + &scope.registry)],
    )
    .expect("Failed to call host.");

    let _ = exec::call_host(&host, &"scp", vec!["-r", &config.path, &host0_path])
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

    let _x =
        exec::call_host(&host, &"ssh", vec![host0, &deploy_command]).expect("Failed to call host.");

    Ok(())
}
