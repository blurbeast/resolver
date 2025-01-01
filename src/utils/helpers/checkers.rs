

use std::io::Error as OutputError;
use std::process::{Command, Output};

pub fn is_node_installed() -> bool {
    let output = Command::new("node").arg("--version").output();

    check_output(output)
}

pub fn is_npm_installed() -> bool {
    let output = Command::new("npm").arg("--version").output();

    check_output(output)
}

pub fn is_nestjs_installed() -> bool {
    let output = Command::new("nest").arg("--version").output();

    check_output(output)
}

pub fn is_php_installed() -> bool {
    let output = Command::new("php").arg("--version").output();

    check_output(output)
}

pub fn is_composer_installed() -> bool {
    let output = Command::new("composer").arg("--version").output();

    check_output(output)
}

pub fn is_scarb_installed() -> bool {
    let output = Command::new("scarb").arg("--version").output();

    check_output(output)
}

pub fn is_forge_installed() -> bool {
    let output = Command::new("forge").arg("--version").output();

    check_output(output)
}

pub fn is_python_installed() -> bool {
    //TODO: Check for python3 for macos
    let output = Command::new("python3").arg("--version").output();

    check_output(output)
}

pub fn is_pip_installed() -> bool {
    let output = Command::new("pip3").arg("--version").output();

    check_output(output)
}

pub fn is_starkli_installed() -> bool {
    let output = Command::new("starkli").arg("--version").output();

    check_output(output)
}

pub fn is_brew_installed() -> bool {
    let output = Command::new("brew").arg("--version").output();

    check_output(output)
}

pub fn is_choco_installed() -> bool {
    let output = Command::new("choco").arg("--version").output();

    check_output(output)
}

pub fn is_nargo_installed() -> bool {
    let output = Command::new("noirup").arg("--version").output();

    check_output(output)
}

pub fn is_solana_cli_installed() -> bool {
    let output = Command::new("solana").arg("--version").output();

    check_output(output)
}

pub fn is_anchor_installed() -> bool {
    let output = Command::new("anchor").arg("--version").output();

    check_output(output)
}

pub fn get_os() -> String {
    let os_family = std::env::consts::OS;

    os_family.to_string()
}

fn check_output(output: Result<Output, OutputError>) -> bool {
    match output {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}