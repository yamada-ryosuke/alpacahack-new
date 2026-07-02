use std::{fs, path::PathBuf};

use anyhow::{Context, Result, ensure};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// alpacahack-newの設定
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// AlpacaHackディレクトリのパス
    pub alpacahack_dir: PathBuf
}

/// 設定を取得する。
pub fn get(alpacahack_new: ProjectDirs) -> Result<Config> {
    let config_path = alpacahack_new.config_local_dir().join("config.toml");
    ensure!(config_path.exists(), "設定ファイルが存在しません。");
    let config_data = fs::read_to_string(config_path).context("設定ファイルが読み込めませんでした。")?;
    let config_data = toml::from_str(&config_data).context("設定ファイルがパースできませんでした。")?;
    Ok(config_data)
}