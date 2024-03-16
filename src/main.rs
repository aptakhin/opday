use std::path::PathBuf;

use clap::{Parser, Subcommand};
use log::debug;

mod config;
mod exec;
mod flow;
use crate::config::DockerComposeFormat;

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

    #[arg(short, long, default_value = "false")]
    quite: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// builds things
    Build {
        /// names
        #[arg(value_name = "NAME")]
        names: Vec<String>,

        /// build args
        #[arg(short, long, value_name = "build-arg")]
        build_arg: Vec<String>,
    },

    /// deploys things
    Deploy {
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

    let config = config::read_configuration("tests/dkrdeliver.test.toml")
        .expect("Could not read configuration.");

    // let scope = &config.environments[0];

    match &cli.command {
        Some(Commands::Build { names, build_arg }) => {
            let f = std::fs::File::open(&config.docker_compose_file).expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _ = flow::build(&config, &format, &names, build_arg);
        }
        Some(Commands::Deploy { names, build_arg }) => {
            let f = std::fs::File::open(&config.docker_compose_file).expect("Could not open file.");
            let format: DockerComposeFormat =
                serde_yaml::from_reader(f).expect("Could not read values.");
            debug!("{:?}", format);

            let _x = flow::deploy(&config, &format, names, build_arg);
        }
        None => {}
    }
    Ok(())
}
