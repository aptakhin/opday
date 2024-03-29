use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::Subcommand;

use serde_yaml::{Mapping, Value};

extern crate term;

use crate::config::{Configuration, DockerComposeFormat};
use crate::exec::{execute_command, RemoteHostCall};

#[derive(Subcommand)]
pub enum DockerProviderCommands {
    /// Build images
    Build {
        /// Names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        // Path to config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Pushes images
    Push {
        /// Names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        // Path to config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Deploys images
    Deploy {
        /// Names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// Path to config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Environment name
        #[arg(short = 'e', long = "env", value_name = "NAME")]
        environment: Option<String>,

        /// Build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Builds and pushes images
    BuildPush {
        /// Names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// Path to config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Environment name
        #[arg(short = 'e', long = "env", value_name = "NAME")]
        environment: Option<String>,

        /// Build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Builds, pushes and deploys images
    BuildPushDeploy {
        /// Names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// Path to config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Environment name
        #[arg(short = 'e', long = "env", value_name = "NAME")]
        environment: Option<String>,

        /// Build args
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
    if config.environments.len() > 1 {
        panic!("Only one environment is supported for now.");
    }

    // Bake docker compose string
    let mut build_command_args: Vec<String> = Vec::new();
    build_command_args.push("compose".to_owned());

    // Add user main docker compose file
    build_command_args.push("-f".to_owned());
    let docker_compose_path = Path::new(&config.path).join(&config.docker_compose_file);
    build_command_args.push(docker_compose_path.to_string_lossy().into_owned());

    // Add user override files
    if config.environments.len() == 1 {
        let scope = &config.environments[0];

        for override_file in &scope.docker_compose_overrides {
            build_command_args.push("-f".to_owned());
            let override_file_path = Path::new(&config.path).join(override_file);
            build_command_args.push(override_file_path.to_string_lossy().into_owned());
        }
    }

    build_command_args.push("build".to_owned());
    let build_command_args2: Vec<&str> = build_command_args.iter().map(|s| s.as_str()).collect();

    let result = execute_command("docker", build_command_args2, build_arg);
    if result.is_err() {
        panic!("Failed to build images ()");
    }
    Ok(())
}

fn push(
    config: &Configuration,
    _format: &DockerComposeFormat,
    _names: &[String],
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.environments.len() > 1 {
        panic!("Only one environment is supported for now.");
    }
    let scope = &config.environments[0];

    let mut build_command_args: Vec<String> = Vec::new();
    build_command_args.push("compose".to_owned());

    build_command_args.push("-f".to_owned());
    let docker_compose_path = Path::new(&config.path).join(&config.docker_compose_file);
    build_command_args.push(docker_compose_path.to_string_lossy().into_owned());

    for override_file in &scope.docker_compose_overrides {
        build_command_args.push("-f".to_owned());
        let docker_compose_override_path = Path::new(&config.path).join(override_file);
        build_command_args.push(docker_compose_override_path.to_string_lossy().into_owned());
    }
    build_command_args.push("push".to_owned());
    let build_command_args2: Vec<&str> = build_command_args.iter().map(|s| s.as_str()).collect();

    let _ = execute_command("docker", build_command_args2, build_arg);
    Ok(())
}

fn deploy(
    config: &Configuration,
    format: &DockerComposeFormat,
    _names: &[String],
    build_arg: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if config.environments.len() > 1 {
        panic!("Only one environment is supported for now.");
    }
    let scope = &config.environments[0];
    let host = RemoteHostCall {
        private_key: scope.ssh_private_key.clone(),
    };

    let generate_file_name = "docker-compose.override-run.yaml";
    let internal_files = Path::new(&config.path).join(".opday-generated");
    let _created = fs::create_dir_all(&internal_files);

    let gitignore_file_path = internal_files.join(".gitignore");

    let mut gitignore_file =
        std::fs::File::create(gitignore_file_path).expect("Could not open file.");
    let _ = gitignore_file
        .write(b"*\n")
        .expect("Could not write values.");
    gitignore_file.flush().expect("Could not flush file.");

    let generated_file = internal_files.join(generate_file_name);

    let run_file = std::fs::File::create(generated_file).expect("Could not open file.");
    let mut run_format = DockerComposeFormat {
        version: format.version.clone(),
        services: Mapping::new(),
        volumes: Some(Mapping::new()),
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

    if scope.hosts.len() > 1 {
        panic!("Only one host is supported for now.");
    }
    let host0 = &scope.hosts[0];
    let host0_path = scope.hosts[0].clone() + ":" + &scope.export_path;

    // scp docker registry auth
    {
        let mut params: Vec<&str> = vec![];
        if host.private_key.is_some() {
            params.push("-i");
            params.push(host.private_key.as_ref().unwrap());
        }
        params.push(&scope.registry_auth_config);
        let reg = host0.clone() + ":" + &scope.registry_export_auth_config;
        params.push(&reg);
        let _ = execute_command("scp", params, &vec![]).expect("Failed to call host.");
    }

    // docker login for registry
    {
        let mut params: Vec<&str> = vec![];
        if host.private_key.is_some() {
            params.push("-i");
            params.push(host.private_key.as_ref().unwrap());
        }
        params.push(host0.as_str());
        let str = "docker login ".to_owned() + &scope.registry;
        params.push(&str);
        let _ = execute_command("ssh", params, &vec![]).expect("Failed to call host.");
    }

    // copy all context docker compose files
    let src_path_ensure_last_slash = Path::new(&config.path).join("");
    let src_path_ensure_last_slash_string = src_path_ensure_last_slash.to_string_lossy();
    {
        let mut params: Vec<&str> = vec![];
        if host.private_key.is_some() {
            params.push("-i");
            params.push(host.private_key.as_ref().unwrap());
        }
        params.push("-r");
        params.push(src_path_ensure_last_slash_string.as_ref());
        params.push(&host0_path);
        let _ = execute_command("scp", params, &vec![]).expect("Failed to call host.");
    }

    let internal_files_export = Path::new(&scope.export_path).join(".opday-generated");

    let mut deploy_command = String::new();
    for build_arg_item in build_arg {
        deploy_command += build_arg_item;
        deploy_command += " ";
    }
    deploy_command += " docker compose -f ";
    let docker_compose_export_path =
        Path::new(&scope.export_path).join(&config.docker_compose_file);
    deploy_command += &docker_compose_export_path.to_string_lossy();

    for override_file in &scope.docker_compose_overrides {
        deploy_command += " -f ";
        let docker_compose_override_export_path = Path::new(&scope.export_path).join(override_file);
        deploy_command += &docker_compose_override_export_path.to_string_lossy();
    }
    deploy_command += " -f ";
    let generate_file_export_path = internal_files_export.join(generate_file_name);
    deploy_command += &generate_file_export_path.to_string_lossy();
    deploy_command += " up -d";

    {
        let mut params: Vec<&str> = vec![];
        if host.private_key.is_some() {
            params.push("-i");
            params.push(host.private_key.as_ref().unwrap());
        }
        params.push(host0);
        params.push(&deploy_command);
        let _ = execute_command("ssh", params, &vec![]).expect("Failed to call host.");
    }

    Ok(())
}

pub fn prepare_config(command: &DockerProviderCommands) -> Option<PathBuf> {
    match &command {
        DockerProviderCommands::Build { config, .. } => config.clone(),
        DockerProviderCommands::Push { config, .. } => config.clone(),
        DockerProviderCommands::Deploy { config, .. } => config.clone(),
        DockerProviderCommands::BuildPush { config, .. } => config.clone(),
        DockerProviderCommands::BuildPushDeploy { config, .. } => config.clone(),
    }
}

pub fn docker_entrypoint(
    command: &DockerProviderCommands,
    _names: &[String],
    global_config: &Configuration,
    _build_arg: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let docker_compose_file_path =
        Path::new(&global_config.path).join(&global_config.docker_compose_file);
    let f = std::fs::File::open(&docker_compose_file_path).unwrap_or_else(|_| {
        panic!(
            "Could not open file {}.",
            &docker_compose_file_path.display()
        )
    });
    let format: DockerComposeFormat = serde_yaml::from_reader(f).expect("Could not read values.");

    match &command {
        DockerProviderCommands::Build {
            names, build_arg, ..
        } => {
            let _ = build(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::Push {
            names, build_arg, ..
        } => {
            let _ = push(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::Deploy {
            names, build_arg, ..
        } => {
            let _ = deploy(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::BuildPush {
            names, build_arg, ..
        } => {
            let _ = build(global_config, &format, names, build_arg);
            let _ = push(global_config, &format, names, build_arg);
        }
        DockerProviderCommands::BuildPushDeploy {
            names, build_arg, ..
        } => {
            let _ = build(global_config, &format, names, build_arg);
            let _ = push(global_config, &format, names, build_arg);
            let _ = deploy(global_config, &format, names, build_arg);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::read_configuration;
    use rstest::fixture;
    use rstest::rstest;

    #[fixture]
    fn simple_config() -> Configuration {
        Configuration {
            path: "tests/01_trivial-backend-no-storage".to_string(),
            docker_compose_file: "docker-compose.yaml".to_string(),
            environments: vec![],
        }
    }

    #[fixture]
    fn simple_docker_compose() -> DockerComposeFormat {
        DockerComposeFormat {
            version: "3.7".to_string(),
            services: Mapping::new(),
            volumes: None,
        }
    }

    #[rstest]
    #[should_panic(
        expected = "No config file found in not-a-file (No such file or directory (os error 2))."
    )]
    fn test_no_config_file() {
        let _ = read_configuration(&PathBuf::from("not-a-file"));
    }

    // does not work on pre-commit
    // #[rstest]
    // fn test_build(simple_config: Configuration, simple_docker_compose: DockerComposeFormat) {
    //     let _ = build(
    //         &simple_config,
    //         &simple_docker_compose,
    //         &vec![],
    //         &vec!["BACKEND_TAG=0.0.1".to_owned()],
    //     );
    // }

    #[rstest]
    #[should_panic(expected = "")]
    fn test_build_no_docker_compose(
        mut simple_config: Configuration,
        simple_docker_compose: DockerComposeFormat,
    ) {
        simple_config.docker_compose_file = "not-a-file".to_string();
        let _ = build(&simple_config, &simple_docker_compose, &vec![], &vec![]);
    }
}
