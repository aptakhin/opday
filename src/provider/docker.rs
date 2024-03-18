use std::path::PathBuf;

use clap::Subcommand;

use log::debug;

use serde_yaml::{Mapping, Value};

use crate::config::{Configuration, DockerComposeFormat};
use crate::exec::{call_host, exec_command, RemoteHostCall};

#[derive(Subcommand)]
pub enum DockerProviderCommands {
    /// Build images
    Build {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Pushes images
    Push {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Deploys images
    Deploy {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
}

fn build(
    config: &Configuration,
    _format: &DockerComposeFormat,
    _names: &[String],
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = &config.environments[0];

    let mut build_command_args: Vec<&str> = Vec::new();
    build_command_args.push("compose");
    build_command_args.push("-f");
    let binding = config.path.clone() + "/docker-compose.yaml";
    build_command_args.push(&binding);
    for override_file in &scope.docker_compose_overrides {
        build_command_args.push("-f");
        build_command_args.push(override_file);
    }
    build_command_args.push("build");

    let _ = exec_command("docker", build_command_args, build_arg);
    Ok(())
}

fn push(
    config: &Configuration,
    _format: &DockerComposeFormat,
    _names: &[String],
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = &config.environments[0];

    let mut build_command_args: Vec<&str> = Vec::new();
    build_command_args.push("compose");
    build_command_args.push("-f");
    let binding = config.path.clone() + "/docker-compose.yaml";
    build_command_args.push(&binding);
    for override_file in &scope.docker_compose_overrides {
        build_command_args.push("-f");
        build_command_args.push(override_file);
    }
    build_command_args.push("push");

    let _ = exec_command("docker", build_command_args, build_arg);
    Ok(())
}

fn deploy(
    config: &Configuration,
    format: &DockerComposeFormat,
    _names: &[String],
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let scope = &config.environments[0];
    let host = RemoteHostCall {
        private_key: scope.ssh_private_key.clone(),
    };

    let generate_file = "tests/.dkr-generated/docker-compose.override-run.yaml";

    let run_file = std::fs::File::create(generate_file).expect("Could not open file.");
    let mut run_format = DockerComposeFormat {
        version: format.version.clone(),
        services: Mapping::new(),
        volumes: Mapping::new(),
    };

    for service in format.services.iter() {
        // override environment
        let mut run_service_map = Mapping::new();
        run_service_map.insert(Value::String((&"build").to_string()), "!reset null".into());
        run_service_map.insert(
            Value::String((&"environment").to_string()),
            Value::Mapping(Mapping::new()),
        );

        run_format.services.insert(
            Value::String(service.0.as_str().unwrap().to_owned()),
            Value::Mapping(run_service_map),
        );
    }
    serde_yaml::to_writer(run_file, &run_format).expect("Could not write values.");

    let host0 = &scope.hosts[0];
    let host0_path = scope.hosts[0].clone() + ":" + &scope.export_path;

    let _ = call_host(
        &host,
        "scp",
        vec![
            &scope.registry_auth_config,
            &(host0.clone() + ":" + &scope.registry_export_auth_config),
        ],
    )
    .expect("Failed to call host.");
    let _ = call_host(
        &host,
        "ssh",
        vec![host0, &("docker login ".to_owned() + &scope.registry)],
    )
    .expect("Failed to call host.");

    let _ = call_host(&host, "scp", vec!["-r", &config.path, &host0_path])
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
    deploy_command += " -f ";
    deploy_command += &generate_file;
    deploy_command += " up -d";

    let _x = call_host(&host, "ssh", vec![host0, &deploy_command]).expect("Failed to call host.");

    Ok(())
}

pub fn prepare_config(command: &DockerProviderCommands) -> Option<PathBuf> {
    match &command {
        DockerProviderCommands::Build { config, .. } => config.clone(),
        DockerProviderCommands::Push { config, .. } => config.clone(),
        DockerProviderCommands::Deploy { config, .. } => config.clone(),
    }
}

pub fn docker_entrypoint(
    command: &DockerProviderCommands,
    _names: &[String],
    global_config: &Configuration,
    _build_arg: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    match &command {
        DockerProviderCommands::Build {
            names, build_arg, ..
        } => {
            let f = std::fs::File::open(&global_config.docker_compose_file)
                .expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _ = build(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::Push {
            names, build_arg, ..
        } => {
            let f = std::fs::File::open(&global_config.docker_compose_file)
                .expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _ = push(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::Deploy {
            names, build_arg, ..
        } => {
            let f = std::fs::File::open(&global_config.docker_compose_file)
                .expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _x = deploy(global_config, &format, names, build_arg);
        }
    }
    Ok(())
}
