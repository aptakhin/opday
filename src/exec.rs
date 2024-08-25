use log::debug;

use std::process::{Command, Stdio};

use std::io::Read;
use std::{thread, time};

pub struct RemoteHostCall {
    pub private_key: Option<String>,
}

#[allow(dead_code)]
pub fn execute_short_command(
    program: &str,
    command: Vec<&str>,
    build_arg: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(program);
    exec_command.args(command.clone());
    for build_arg_item in build_arg {
        let parts: Vec<&str> = build_arg_item.splitn(2, '=').collect();
        if parts.len() < 2 {
            panic!("Invalid build-arg without `=`: `{}`", build_arg_item)
        }
        exec_command.env(parts[0], parts[1]);
    }

    // TODO: correct quoting of command args
    let command_str = String::new() + program + " " + &command.join(" ");

    debug!(
        "Start command: {} envs:{:?}",
        command_str,
        &exec_command.get_envs(),
    );
    let output = exec_command.output().expect("failed to execute process");

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    debug!(
        "Executed command: {:?} envs:{:?} status: {:?} out: {:?} err: {:?}",
        command_str,
        &exec_command.get_envs(),
        status,
        stdout,
        stderr
    );
    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Command failed: {:?} envs:{:?} status:{:?} stdout:{:?} stderr:{:?}",
                command_str,
                &exec_command.get_envs(),
                status,
                stdout,
                stderr
            ),
        )));
    }
    Ok(stdout)
}

pub fn execute_command(
    program: &str,
    command: Vec<&str>,
    build_arg: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut exec_command = Command::new(program);
    exec_command.args(command.clone());
    for build_arg_item in build_arg {
        let parts: Vec<&str> = build_arg_item.splitn(2, '=').collect();
        if parts.len() < 2 {
            panic!("Invalid build-arg without `=`: `{}`", build_arg_item)
        }
        exec_command.env(parts[0], parts[1]);
    }

    // TODO: correct quoting of command args
    let command_str = String::new() + program + " " + &command.join(" ");
    debug!(
        "Start command: {} envs: {:?} cmd: {}",
        program,
        &exec_command.get_envs(), // TODO: filter out secrets
        &command_str,
    );

    let mut process = exec_command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    std::thread::spawn(move || {
        let mut stdout = stdout;
        let mut stderr = stderr;
        let mut closing = false;
        loop {
            let mut buffer = [0; 1024];
            let n = stdout.read(&mut buffer).unwrap();
            if n == 0 {
                closing = true;
            }
            if !closing {
                let s = String::from_utf8_lossy(&buffer[..n]);

                debug!("out: {}", s.replace('\n', "\\n"));
            }

            let mut buffer = [0; 1024];
            let n = stderr.read(&mut buffer).unwrap();
            if closing && n == 0 {
                debug!("finish-finish");
                break;
            }
            let s = String::from_utf8_lossy(&buffer[..n]);

            if n != 0 {
                debug!("err: {}", s.replace('\n', "\\n"));
            }

            thread::sleep(time::Duration::from_secs(1));
        }
    });

    loop {
        let st = process.try_wait()?;
        match st {
            None => debug!("still running"),
            Some(status) => debug!("exited with: {}", status),
        }
        if st.is_some() {
            break;
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    let output = exec_command.output().expect("failed to execute process");

    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    debug!(
        "Executed command: {} status: {:?} cmd: {}",
        program, status, &command_str
    );
    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Command failed: {:?} envs:{:?} cmd:{:?} status:{:?} stdout:{:?} stderr:{:?}",
                program,
                &exec_command.get_envs(),
                &command_str,
                status,
                stdout,
                stderr
            ),
        )));
    }
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_execute_command() {
        let _ = execute_command("echo", vec!["hello"], &vec![]).unwrap();
        assert!(true)
    }
}
