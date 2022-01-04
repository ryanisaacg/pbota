use anyhow::{Context, Result};
use crate::Modifier;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
struct MoveList {
    moves: HashMap<String, Move>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Move {
    preamble: Option<String>,
    postamble: Option<String>,
    options: Vec<(Matcher, String)>,
    pub stat: Option<(String, i32)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Matcher {
    Less(i32),
    Range(i32, i32),
    Greater(i32),
}

pub fn get_move(name: &str) -> Result<Move> {
    Ok(read_config()?.moves.get(name).context("Move not found")?.clone())
}


pub fn get_move_text(mv: Move, roll: i32) -> Result<String> {
    let mut message = mv.preamble.unwrap_or(String::new());
    for (matcher, text) in mv.options.into_iter() {
        let meets_bound = match matcher {
            Matcher::Less(bound) => roll <= bound,
            Matcher::Range(lower, upper) => roll >= lower && roll <= upper,
            Matcher::Greater(bound) => roll >= bound,
        };
        if meets_bound {
            message = format!("{}\n{}", message, text);
        }
    }
    message = format!("{}\n{}", message, mv.postamble.unwrap_or(String::new()));

    Ok(message)
}

fn read_config() -> Result<MoveList> {
    // TODO: find a specific file?
    Ok(match std::fs::read_to_string("moves.json") {
        Ok(file) => serde_json::from_str::<MoveList>(&file)?,
        Err(_) => MoveList::default(),
    })
}

fn write_config(config: MoveList) -> Result<()> {
    use std::io::Write;
    
    // TODO: find a specific file?
    let data = serde_json::to_string(&config)?;
    let f = std::fs::File::create("moves.json")?;
    let mut f = std::io::BufWriter::new(f);
    f.write_all(data.as_bytes())?;

    Ok(())
}
