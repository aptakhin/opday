use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use config::Configuration;
use log::debug;

mod config;
mod exec;
mod provider;

use crate::provider::docker::{docker_entrypoint, prepare_config, DockerProviderCommands};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Identity file (private key) for ssh
    #[arg(short = 'i', long, value_name = "FILE")]
    ssh_private_key: Option<PathBuf>,

    /// Verbose level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    provider: Option<Providers>,
}

#[derive(Subcommand)]
enum Providers {
    /// builds images
    Docker {
        #[command(subcommand)]
        command: DockerProviderCommands,

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

#[allow(clippy::single_match)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    env_logger::init();

    let default_config_file = Path::new("dkrdeliver.toml");

    let mut global_config: Option<Configuration> = None;
    if cli.config.is_some() {
        debug!("Using config file: {:?}", cli.config);
        global_config = Some(
            config::read_configuration(&cli.config.unwrap())
                .expect("Could not read configuration."),
        );
    }

    match &cli.provider {
        Some(Providers::Docker {
            command,
            names,
            config,
            build_arg,
        }) => {
            if config.is_some() {
                debug!("Using config file: {:?}", config);
                global_config = Some(
                    config::read_configuration(
                        &<std::option::Option<PathBuf> as Clone>::clone(config).unwrap(),
                    )
                    .expect("Could not read configuration."),
                );
            }

            let config_after_subsubcommand = prepare_config(command);
            if config_after_subsubcommand.is_some() {
                debug!("Using config file: {:?}", config_after_subsubcommand);
                global_config = Some(
                    config::read_configuration(&config_after_subsubcommand.unwrap())
                        .expect("Could not read configuration."),
                );
            }

            if global_config.is_none() && Path::exists(default_config_file) {
                debug!(
                    "Using default config file: {}",
                    default_config_file.display()
                );
                global_config = Some(
                    config::read_configuration(default_config_file)
                        .expect("Could not read configuration."),
                );
            }

            if global_config.is_none() {
                panic!("No configuration found. Use `--config`.");
            }
            let global_config_unwrap = global_config.unwrap();

            let _ = docker_entrypoint(command, names, &global_config_unwrap, build_arg);
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        args,
        case::just_docker(vec!["docker"]),
        case::just_docker_build(vec!["", "docker", "build"]),
        case::config_abefore_sub_command(vec!["", "--config", "myconfig", "docker", "build"]),
        case::config_after_sub_command(vec!["", "docker", "--config", "myconfig", "build"]),
        case::config_after_sub_sub_command(vec!["", "docker", "build", "--config", "myconfig"]),
        case::config_after_sub_sub_command_plus_build_arg(vec!["", "docker", "build", "--config", "myconfig", "--build-arg", "BACKEND_TAG=0.0.1"]),
    )]
    fn test_config_for_any_order(args: Vec<&str>) {
        assert!(Cli::try_parse_from(args).is_ok());
    }
}
