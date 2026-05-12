use crate::AppResult;
use anyhow::anyhow;
use config::{Config, File, Source};
use std::path::Path;

pub fn read_config<'de, T: serde::Deserialize<'de>>(cfg_file: &str) -> AppResult<T> {
    let (cfg, _) = read_config_with_required(cfg_file, true, &[])?;
    Ok(cfg)
}

pub fn read_config_default<'de, T: serde::Deserialize<'de>>(cfg_file: &str) -> AppResult<T> {
    let (cfg, def) = read_config_with_required(cfg_file, false, &[])?;
    if def {
        log::warn!(
            "The configuration for  '{cfg_file}' does not exist, default values will be used!"
        );
    }
    Ok(cfg)
}

pub fn read_config_with<'de, T: serde::Deserialize<'de>>(
    cfg_file: &str,
    env_list_keys: &[&str],
) -> AppResult<T> {
    let (cfg, _) = read_config_with_required(cfg_file, true, env_list_keys)?;
    Ok(cfg)
}

pub fn read_config_default_with<'de, T: serde::Deserialize<'de>>(
    cfg_file: &str,
    env_list_keys: &[&str],
) -> AppResult<T> {
    let (cfg, def) = read_config_with_required(cfg_file, false, env_list_keys)?;
    if def {
        log::warn!(
            "The configuration for  '{cfg_file}' does not exist, default values will be used!"
        );
    }
    Ok(cfg)
}

pub fn read_config_with_required<'de, T: serde::Deserialize<'de>>(
    cfg_file: &str,
    required: bool,
    env_list_keys: &[&str],
) -> AppResult<(T, bool)> {
    let path = Path::new(cfg_file);
    if !path.is_file() {
        return Err(anyhow!(format!("not found: {cfg_file}")));
    }
    let builder = Config::builder().add_source(File::from(path).required(required));
    let mut env = config::Environment::with_prefix(&format!("gateway_"));
    if !env_list_keys.is_empty() {
        env = env.try_parsing(true).list_separator(" ");
        for key in env_list_keys {
            env = env.with_list_parse_key(key);
        }
    }

    let builder = builder.add_source(env);
    let s = builder.build()?;
    let count = s.collect()?.len();
    Ok((s.try_deserialize::<T>()?, count == 0))
}
