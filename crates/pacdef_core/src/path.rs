use std::{env, path::PathBuf};

use anyhow::{Context, Result};

const CONFIG_FILE_NAME: &str = "pacdef.yaml";

/// Get the group directory where all group files are located. This is
/// `$XDG_CONFIG_HOME/pacdef/groups`, which defaults to `$HOME/.config/pacdef/groups`.
pub fn get_pacdef_group_dir() -> Result<PathBuf> {
    let mut result = get_pacdef_base_dir().context("getting pacdef base dir")?;
    result.push("groups");
    Ok(result)
}

pub fn get_pacdef_base_dir() -> Result<PathBuf> {
    let mut dir = get_xdg_config_home().context("getting XDG_CONFIG_HOME")?;
    dir.push("pacdef");
    Ok(dir)
}

fn get_xdg_config_home() -> Result<PathBuf> {
    if let Ok(config) = env::var("XDG_CONFIG_HOME") {
        Ok(config.into())
    } else {
        let mut config = get_home_dir().context("falling back to $HOME/.config")?;
        config.push(".config");
        Ok(config)
    }
}

pub fn get_home_dir() -> Result<PathBuf> {
    Ok(env::var("HOME").context("getting $HOME variable")?.into())
}

/// Get the group directory where all group files are located. This is
/// `$XDG_CONFIG_HOME/pacdef/pacdef.yaml`.
pub fn get_config_path() -> Result<PathBuf> {
    let mut file = get_pacdef_base_dir().context("getting pacdef base dir for config file")?;
    file.push(CONFIG_FILE_NAME);
    Ok(file)
}
