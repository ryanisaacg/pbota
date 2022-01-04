use anyhow::{Context as AContext, Result};
use fastrand::i32;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::env;

mod char;
mod moves;
mod parameters;
use parameters::{Parameters, Modifier};

#[group]
#[commands(m, r, roll, char)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    println!("{}", token);
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn r(ctx: &Context, msg: &Message) -> CommandResult {
    do_roll(ctx, msg).await
}

#[command]
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    do_roll(ctx, msg).await
}

async fn do_roll(ctx: &Context, msg: &Message) -> CommandResult {
   let contents = match msg.content.find(' ') {
       Some(idx) => msg.content.split_at(idx + 1).1,
       None => "",
    };
    let param = parameters::parameters(contents);
    let (_, message) = calculate_roll(msg.author.id.0, param)?;
    msg.reply(ctx, message).await?;

    Ok(())
}

fn calculate_roll(user: u64, Parameters { modifier, character, hope, despair }: Parameters) -> Result<(i32, String)> {
    use std::cmp::{max, min};

    let d1 = i32(1..6);
    let d2 = i32(1..6);

    if hope && despair {
        // TODO: wording
        return None.context("You can't roll with advantage AND disadvantage!");
    }

    let (mod_num, mod_string) = match modifier {
        Some(Modifier::Number(num)) => (num, format!("+{}", num)),
        Some(Modifier::Stat { sign, stat }) => {
            let value = sign * char::get_stat(character, user, stat)?;
            let sign = if sign > 0 { "+" } else { "-" };
            (value, format!("{}{} [{}]", sign, stat, value))
        }
        None => (0, "".to_owned()),
    };

    let mut sum = d1 + d2 + mod_num;
    let mut dice = format!("Rolled {} and {}", d1, d2);

    if hope {
        let d3 = i32(1..6);
        let least = min(d1, min(d2, d3));
        sum += d3 - least;
        dice = format!("Rolled {}, {} and {}; dropped {}", d1, d2, d3, least);
    } else if despair {
        let d3 = i32(1..6);
        let greatest = max(d1, max(d2, d3));
        sum += d3 - greatest;
        dice = format!("Rolled {}, {} and {}; dropped {}", d1, d2, d3, greatest);
    }

    Ok((sum, format!("**{}**, ({}){}", sum, dice, mod_string)))
}


#[command]
async fn m(ctx: &Context, msg: &Message) -> CommandResult {
    let mut words = msg.content.split(' ');
    words.next(); // strip leading
    let move_name = words.next().context("Move name required")?;
    let mv = moves::get_move(move_name)?;
    let parameters = words.collect::<Vec<&str>>().join(" ");
    let mut param = parameters::parameters(&parameters);
    if let Some((ref stat, sign)) = mv.stat {
        if param.modifier.is_none() {
            param.modifier = Some(Modifier::Stat { stat, sign });
        }
    }
    let (result, roll_text) = calculate_roll(msg.author.id.0, param)?;
    let text = moves::get_move_text(mv, format!("You rolled a {}", roll_text), result)?;
    msg.reply(ctx, text).await?;
    Ok(())
}

#[command]
async fn char(ctx: &Context, msg: &Message) -> CommandResult {
    let mut words = msg.content.split(' ');
    words.next(); // strip leading
    match words.next() {
        Some("new") => {
            let name = words.next().context("No name provided")?;
            char::new(name)?;
            msg.reply(ctx, format!("Character with name {} created", name)).await?;
        }
        Some("choose") => {
            let name = words.next().context("No name provided")?;
            char::choose(msg.author.id.0, name)?;
            msg.reply(ctx, format!("Set up to roll as {}", name)).await?;
        }
        Some("stat") => {
            let name = words.next().context("No name provided")?;
            let stat = words.next().context("No stat provided")?;
            let val = words.next().context("No value provided")?.parse::<i32>()?;
            char::set_stat(name, stat, val)?;
            msg.reply(ctx, format!("{}'s {} stat is now {}", name, stat, val)).await?;
        }
        _ => {
            msg.reply(ctx, "Command not recognized").await?;
            return Ok(());
        }
    }
    Ok(())
}




