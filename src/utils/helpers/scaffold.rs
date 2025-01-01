use std::error::Error;
use std::fs;
use std::io::Error as OutputError;
use std::process::{Command, Output};
use crate::utils::helpers::checkers::*;

pub fn create_react_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-react-app", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_react_app_with_typescript(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args([
                "create-react-app",
                project_name.as_str(),
                "--template",
                "typescript",
            ])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_hardhat_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        fs::create_dir_all(project_name.as_str())?;

        Command::new("npm")
            .args(["init", "--yes"])
            .current_dir(project_name.as_str())
            .spawn()?
            .wait()?;

        Command::new("npx")
            .args(["hardhat", "init"])
            .current_dir(project_name.as_str())
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_nestjs_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_nestjs_installed() {
        if !is_npm_installed() {
            return Err("You don't have npm installed".into());
        }

        Command::new("npm")
            .args(["i", "-g", "@nestjs/cli"])
            .spawn()?
            .wait()?;
    }

    Command::new("nest")
        .args(["new", project_name.as_str()])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn create_laravel_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_php_installed() && !is_composer_installed() {
        return Err("You don't have PHP or Composer installed".into());
    } else {
        println!("Creating Laravel project: {}", project_name);

        Command::new("composer")
            .args(["create-project", "laravel/laravel", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_next_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-next-app@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_new_foundry_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_forge_installed() {
        return Err("You don't have Forge installed".into());
    } else {
        Command::new("forge")
            .args(["init", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_django_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_python_installed() && !is_pip_installed() {
        return Err("You don't have Python installed".into());
    } else {
        Command::new("django-admin")
            .args(["startproject", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_vue_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args(["create", "vue@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_vite_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args(["create", "vite@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_noir_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_nargo_installed() {
        return Err("You don't have nargo installed".into());
    } else {
        Command::new("nargo")
            .args(["new", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_starknet_foundry_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_forge_installed() {
        return Err("You don't have Forge installed".into());
    } else {
        Command::new("snforge")
            .args(["init", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_expo_app(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npx")
            .args(["create-expo-app", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_adonis_project(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_npm_installed() {
        return Err("You don't have npm installed".into());
    } else {
        Command::new("npm")
            .args(["init", "adonisjs@latest", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_anchor_project_ts(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_solana_cli_installed() {
        return Err("You don't have the Solana CLI installed".into());
    } else if !is_anchor_installed() {
        return Err("You don't have Anchor installed".into());
    } else if !is_node_installed() {
        return Err("You don't have Node installed".into());
    } else {
        Command::new("anchor")
            .args(["init", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}

pub fn create_anchor_project_rs(project_name: String) -> Result<(), Box<dyn Error>> {
    if !is_solana_cli_installed() {
        return Err("You don't have the Solana CLI installed".into());
    } else if !is_anchor_installed() {
        return Err("You don't have Anchor installed".into());
    } else {
        Command::new("anchor")
            .args(["init", "--test-template", "rust", project_name.as_str()])
            .spawn()?
            .wait()?;

        Ok(())
    }
}