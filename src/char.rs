use anyhow::{Context, Result};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// user_to_char needs to be validated

#[derive(Default, Deserialize, Serialize)]
pub struct CharConfig {
    user_to_char: HashMap<u64, String>,
    characters: HashMap<String, CharInfo>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct CharInfo {
    stats: HashMap<String, i32>,
}

pub fn new(name: &str) -> Result<()> {
    let mut cfg = read_config()?;
    cfg.characters.insert(name.to_owned(), CharInfo::default());
    write_config(cfg)?;

    Ok(())
}

pub fn choose(id: u64, name: &str) -> Result<()> {
    let mut cfg = read_config()?;
    cfg.user_to_char.insert(id, name.to_owned());
    write_config(cfg)?;

    Ok(())
}

pub fn set_stat(name: &str, stat: &str, val: i32) -> Result<()> {
    let mut cfg = read_config()?;
    cfg.characters.get_mut(name).context("Character not found")?.stats.insert(stat.to_owned(), val);
    write_config(cfg)?;

    Ok(())
}

pub fn get_stat(name: Option<&str>, user: u64, stat: &str) -> Result<i32> {
    let cfg = read_config()?;
    let name = if let Some(name) = name {
        name
    } else {
        cfg.user_to_char.get(&user).context("No character chosen")?
    };
    let val = cfg.characters.get(name).context("Character not found")?.stats.get(stat).context("Stat not set")?;

    Ok(*val)
}

fn read_config() -> Result<CharConfig> {
    // TODO: find a specific file?
    Ok(match std::fs::read_to_string("characters.json") {
        Ok(file) => serde_json::from_str::<CharConfig>(&file)?,
        Err(_) => CharConfig::default(),
    })
}

fn write_config(config: CharConfig) -> Result<()> {
    use std::io::Write;
    
    // TODO: find a specific file?
    let data = serde_json::to_string(&config)?;
    let f = std::fs::File::create("characters.json")?;
    let mut f = std::io::BufWriter::new(f);
    f.write_all(data.as_bytes())?;

    Ok(())
}