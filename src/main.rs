use anyhow::Context as AContext;
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
mod parameters;
use parameters::{Parameters, Modifier, ModifierValue};

#[group]
#[commands(r, roll, char)]
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
    use std::cmp::{max, min};

    let d1 = i32(1..6);
    let d2 = i32(1..6);

    let Parameters { modifier, character, hope, despair } = parameters::parameters(msg);

    if hope && despair {
        // TODO: wording
        msg.reply(ctx, "You can't roll with advantage AND disadvantage!").await?;
        return Ok(());
    }

    let mod_num = match modifier {
        Some(Modifier { ref value }) => match value {
            ModifierValue::Number(num) => *num,
            ModifierValue::Stat { sign, stat } => sign * char::get_stat(character, msg.author.id.0, stat)?,
        },
        None => 0,
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

    msg.reply(ctx, format!("**{}**, ({}){}", sum, dice, modifier.map(|_| format!("+{}", mod_num)).unwrap_or("".to_owned()))).await?;
    

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


