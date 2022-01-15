use anyhow::{Context, Result};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
struct MoveList {
    moves: HashMap<String, Move>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Move {
    pub preamble: Option<String>,
    pub postamble: Option<String>,
    pub options: Vec<(Matcher, String)>,
    pub stat: Option<(String, i32)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Matcher {
    Less(i32),
    Range(i32, i32),
    Greater(i32),
}

pub fn get_moves() -> Result<Vec<String>> {
    let mut moves = read_config()?.moves.keys().cloned().collect::<Vec<String>>();
    moves.sort();
    Ok(moves)
}

pub fn get_move(name: &str) -> Result<Move> {
    Ok(read_config()?.moves.get(name).context("Move not found")?.clone())
}


pub fn get_move_text(mv: Move, dice_string: String, roll: i32) -> Result<String> {
    let mut message = mv.preamble.unwrap_or(String::new());
    if message.len() != 0 {
        message.push_str("\n\n");
    }
    message.push_str(&dice_string);
    message.push_str("\n");
        
    for (matcher, text) in mv.options.into_iter() {
        let meets_bound = match matcher {
            Matcher::Less(bound) => roll <= bound,
            Matcher::Range(lower, upper) => roll >= lower && roll <= upper,
            Matcher::Greater(bound) => roll >= bound,
        };
        if meets_bound {
            message.push('\n');
            message.push_str(&text);
        }
    }
    message.push('\n');
    message.push_str(&mv.postamble.unwrap_or(String::new()));

    Ok(message)
}

fn read_config() -> Result<MoveList> {
    // TODO: find a specific file?
    Ok(match std::fs::read_to_string("moves.json") {
        Ok(file) => serde_json::from_str::<MoveList>(&file)?,
        Err(_) => MoveList::default(),
    })
}

pub fn add_move(name: String, mov: Move) -> Result<()> {
    use std::io::Write;

    let mut config = read_config()?;
    config.moves.insert(name, mov);

    let data = serde_json::to_string(&config)?;
    let f = std::fs::File::create("moves.json")?;
    let mut f = std::io::BufWriter::new(f);
    f.write_all(data.as_bytes())?;

    Ok(())
}

