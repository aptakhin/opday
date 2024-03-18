use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::Configuration;
use log::debug;

mod config;
mod exec;
mod flow;
use crate::config::DockerComposeFormat;


#[derive(Subcommand)]
pub enum DockerProviderCommands {
    /// Build images
    Build {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Pushes images
    Push {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
    /// Deploys images
    Deploy {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
}

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

    #[arg(short, long, default_value = "false")]
    quite: bool,

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

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    env_logger::init();

    let mut config: Option<Configuration> = None;
    if cli.config.is_some() {
        debug!("Using config file: {:?}", cli.config);
        config = Some(
            config::read_configuration(&cli.config.unwrap())
                .expect("Could not read configuration."),
        );
    }

    match &cli.provider {
        Some(Providers::Docker { command, names: _, build_arg: _ }) => {
            match &command {
                DockerProviderCommands::Build { names, build_arg } => {
                    if config.is_none() {
                        panic!("No configuration found. Use `--config`.");
                    }
                    let conf = config.unwrap();
                    let f = std::fs::File::open(&conf.docker_compose_file).expect("Could not open file.");
                    let format: DockerComposeFormat =
                        serde_yaml::from_reader(f).expect("Could not read values.");
                    debug!("{:?}", format);

                    let _ = flow::build(&conf, &format, names, build_arg);
                }
                DockerProviderCommands::Push { names, build_arg } => {
                    if config.is_none() {
                        panic!("No configuration found. Use `--config`.");
                    }
                    let conf = config.unwrap();
                    let f = std::fs::File::open(&conf.docker_compose_file).expect("Could not open file.");
                    let format: DockerComposeFormat =
                        serde_yaml::from_reader(f).expect("Could not read values.");
                    debug!("{:?}", format);

                    let _ = flow::push(&conf, &format, names, build_arg);
                }
                DockerProviderCommands::Deploy { names, build_arg } => {
                    if config.is_none() {
                        panic!("No configuration found. Use `--config`.");
                    }
                    let conf = config.unwrap();

                    let f = std::fs::File::open(&conf.docker_compose_file).expect("Could not open file.");
                    let format: DockerComposeFormat =
                        serde_yaml::from_reader(f).expect("Could not read values.");
                    debug!("{:?}", format);

                    let _x = flow::deploy(&conf, &format, names, build_arg);
                }
            }
        }
        _ => {}
    }

    Ok(())
}
