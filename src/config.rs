// pub mod config;

use log::debug;
use toml::Table;

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
    // current_scope: Scope,
    pub environments: Vec<Scope>,
}

// pub mod config {

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

pub fn read_configuration(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
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

// }
