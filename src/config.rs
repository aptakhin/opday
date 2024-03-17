use std::path::Path;

use log::debug;
use toml::Table;

use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;


#[derive(Debug, Serialize, Deserialize)]
pub struct DockerComposeFormat {
    pub version: String,
    pub services: Mapping,
    pub volumes: Mapping,
}

pub struct Scope {
    pub hosts: Vec<String>,
    pub registry: String,
    pub registry_auth_config: String,
    pub registry_export_auth_config: String,
    pub docker_compose_overrides: Vec<String>,
    pub ssh_private_key: Option<String>,
}

pub struct Configuration {
    pub path: String,
    pub docker_compose_file: String,
    pub environments: Vec<Scope>,
}

fn get_string_value<'a>(current: &'a Table, base: &'a Table, key: &str, required: bool) -> Option<String> {
    if current.contains_key(key) {
        return Some(current[key].as_str().unwrap().to_string());
    } else if base.contains_key(key) {
        return Some(base[key].as_str().unwrap().to_string());
    } else if required {
        panic!("Can't find config value for `{}` key.", key);
    }
    None
}

fn get_string_array_value<'a>(
    current: &'a Table,
    base: &'a Table,
    key: &str,
    required: bool,
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
    } else if required {
        panic!("Can't find config value for `{}` key.", key);
    }
    None
}

fn push_parsing_scope(current: &Table, base: &Table) -> Scope {
    let registry = get_string_value(current, base, "registry", true);
    let hosts = get_string_array_value(current, base, "hosts", true);
    let registry_auth_config = get_string_value(current, base, "registry_auth_config", true);
    let registry_export_auth_config =
        get_string_value(current, base, "registry_export_auth_config", true);
    let docker_compose_overrides =
        get_string_array_value(current, base, "docker_compose_overrides", true);
    let ssh_private_key = get_string_value(current, base, "ssh_private_key", false);

    return Scope {
        hosts: hosts.unwrap(),
        registry: registry.unwrap(),
        registry_auth_config: registry_auth_config.unwrap(),
        registry_export_auth_config: registry_export_auth_config.unwrap(),
        docker_compose_overrides: docker_compose_overrides.unwrap(),
        ssh_private_key,
    };
}

pub fn read_configuration_raw(content: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let cfg_parse: Result<Table, toml::de::Error> = content.parse();

    if cfg_parse.is_err() {
        panic!("Config parsing error: {:?}", cfg_parse.err().unwrap())
    }

    let cfg: Table = cfg_parse.unwrap();

    let mut environments: Vec<Scope> = vec![];
    let mut base_scope = Table::new();

    if !cfg.contains_key("environments") {
        panic!("Invalid config file. No environments found.");
    }

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

    if !cfg.contains_key("path") {
        panic!("Invalid config file. No path found.");
    }
    if !cfg.contains_key("docker_compose_file") {
        panic!("Invalid config file. No docker_compose_file found.");
    }
    let config: Configuration = Configuration {
        path: cfg["path"].as_str().unwrap().to_string(),
        docker_compose_file: cfg["docker_compose_file"].as_str().unwrap().to_string(),
        environments: environments,
    };
    Ok(config)
}

pub fn read_configuration(path: &Path) -> Result<Configuration, Box<dyn std::error::Error>> {
    let path = std::path::Path::new(&path);
    let file = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    read_configuration_raw(&file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid config file. No environments found.")]
    fn test_empty_configuration() {
        let toml_data = r#"
        "#;
        let config = read_configuration_raw(&toml_data);
        assert_eq!(config.is_ok(), true);
    }

    #[test]
    #[should_panic(expected = "Invalid config file. No path found.")]
    fn test_no_path() {
        let toml_data = r#"
        [environments]
        "#;
        let config = read_configuration_raw(&toml_data);
        assert_eq!(config.is_ok(), true);
    }

    #[test]
    #[should_panic(expected = "Invalid config file. No docker_compose_file found.")]
    fn test_no_docker_compose_file() {
        let toml_data = r#"
        path = "path"
        [environments]
        "#;
        let config = read_configuration_raw(&toml_data);
        assert_eq!(config.is_ok(), true);
    }

    #[test]
    fn test_environment_order_load() {
        let toml_data = r#"
        path = "path"
        docker_compose_file = "file"

        [environments]
        ssh_private_key = "akey"
        registry = "aregistry"
        registry_export_auth_config = "aexport_auth"
        docker_compose_overrides = ["aoverride"]
        hosts = ["ahost"]

        [environments.b]
        ssh_private_key = "bkey"
        registry_auth_config = "bauth"
        hosts = ["bhost"]
        "#;

        let config_result = read_configuration_raw(&toml_data);
        assert_eq!(config_result.is_ok(), true);
        let config = config_result.unwrap();
        assert_eq!(config.environments[0].ssh_private_key, Some("bkey".to_string()));
        assert_eq!(config.environments[0].registry, "aregistry".to_string());
        assert_eq!(config.environments[0].registry_auth_config, "bauth".to_string());
        assert_eq!(config.environments[0].docker_compose_overrides, vec!["aoverride"]);
        assert_eq!(config.environments[0].hosts, vec!["bhost"]);
    }
}
