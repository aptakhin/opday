use std::{ffi::OsStr, process::Command};

use log::debug;

pub struct RemoteHostCall {
    pub private_key: Option<String>,
}

pub fn exec_command(
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
    debug!(
        "Start command: {:?} envs:{:?} args:{:?}",
        program,
        &exec_command.get_envs(),
        &exec_command.get_args(),
    );
    let output = exec_command.output().expect("failed to execute process");

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    debug!(
        "Executed command: {:?} envs:{:?} args:{:?} {:?} {:?} {:?}",
        program,
        &exec_command.get_envs(),
        &exec_command.get_args(),
        status,
        stdout,
        stderr
    );
    Ok(stdout)
}

pub fn call_host(
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
