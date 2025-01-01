
pub fn install_brew() -> Result<(), Box<dyn Error>> {
    println!("Installing Homebrew...");

    let script_url = "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh";
    let command = format!("curl -fsSL {} | /bin/bash", script_url);

    Command::new("sh").arg("-c").arg(command).output()?;

    Ok(())
}

pub fn install_choco() -> Result<(), Box<dyn Error>> {
    println!("Installing Chocolatey...");

    let powershell_script = r#"
        Set-ExecutionPolicy Bypass -Scope Process -Force;
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
        iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
    "#;

    Command::new("powershell")
        .arg("-Command")
        .arg(powershell_script)
        .output()?;

    Ok(())
}

pub fn install_node() -> Result<(), Box<dyn Error>> {
    println!("Installing the Latest Version of Node...");

    // let os_family = std::env::consts::OS;
    let os_family = get_os();

    match os_family.as_str() {
        "linux" => install_node_linux(),
        "windows" => install_node_windows(),
        "macos" => install_node_macos(),
        _ => panic!("Unsupported OS"),
    }
}

pub fn install_node_linux() -> Result<(), Box<dyn Error>> {
    println!("Installing Node.js on Linux...");

    Command::new("sudo")
        .arg("apt-get")
        .args(["update"])
        .status()?;

    Command::new("sudo")
        .arg("apt-get")
        .args(["install", "-y", "nodejs"])
        .status()?;

    Ok(())
}

pub fn install_node_macos() -> Result<(), Box<dyn Error>> {
    println!("Installing Node.js on macOS...");

    if is_brew_installed() {
        Command::new("brew").args(["install", "node"]).status()?;
    }

    Ok(())
}

pub fn install_node_windows() -> Result<(), Box<dyn Error>> {
    println!("Installing Node.js on Windows...");

    if is_choco_installed() {
        Command::new("choco")
            .args(["install", "nodejs", "-y"])
            .status()?;
    }

    Ok(())
}

pub fn install_scarb() -> Result<(), Box<dyn Error>> {
    println!("Installing the Latest Version of Scarb...");

    let install_cmd =
        "curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh";

    Command::new("sh").arg("-c").arg(install_cmd).output()?;

    Ok(())
}

pub fn install_starkli() -> Result<(), Box<dyn Error>> {
    println!("Installing the Latest Version of Starkli...");

    Command::new("sh")
        .arg("-c")
        .arg("curl https://get.starkli.sh | sh")
        .output()?;

    Command::new("sh").arg("-c").arg("starkliup").output()?;

    Ok(())
}

pub fn install_composer() {}

pub fn install_forge() -> Result<(), Box<dyn Error>> {
    println!("Installing the Latest Version of Foundry...");

    Command::new("sh")
        .arg("-c")
        .arg("curl -L https://foundry.paradigm.xyz | bash")
        .output()?;

    Command::new("sh").arg("-c").arg("foundryup").output()?;

    Ok(())
}

pub fn install_nargo() -> Result<(), Box<dyn Error>> {
    println!("Installing the Latest Version of Noir...");

    Command::new("sh")
        .arg("-c")
        .arg("curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash")
        .output()?;

    Command::new("sh").arg("-c").arg("noirup").output()?;

    Ok(())
}

pub fn install_snforge(version: String) -> Result<(), Box<dyn Error>> {
    if version.eq("latest") {
        println!("Installing the Latest Version of Starknet Foundry, Please wait...");

        Command::new("sh")
            .arg("-c")
            .arg("curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh")
            .output()?;

        Command::new("sh").arg("-c").arg("snfoundryup").output()?;

        Ok(())
    } else {
        println!(
            "Installing Version {} of Starknet Foundry, Please wait...",
            version
        );

        Command::new("sh")
            .args(["-c", "curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh"])
            .output()?;

        Command::new("sh")
            .args(["-c", "snfoundryup", "-v", version.as_str()])
            .output()?;

        Ok(())
    }
}

pub fn create_rainbowkit_wagmi_next_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args([
                "init",
                "@rainbow-me/rainbowkit@latest",
                project_name.as_str(),
            ])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn install_solana_cli() -> Result<(), Box<dyn Error>> {
    if is_solana_cli_installed() {
        println!("You already have the Solana CLI installed");
    } else {
        println!("Installing the Latest Version of the Solana CLI...");
        Command::new("sh")
            .arg("-c")
            .arg("curl -sSfL https://release.anza.xyz/stable/install | sh")
            .status()?;
    }
    Ok(())
}
pub fn install_anchor() -> Result<(), Box<dyn Error>> {
    println!("Installing Anchor...");

    Command::new("cargo")
        .args([
            "install",
            "--git",
            "https://github.com/coral-xyz/anchor",
            "avm",
            "--force",
        ])
        .spawn()?
        .wait()?;

    Ok(())
}