use anyhow::{Context as AContext, Result, bail};
use fastrand::i32;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
    id::UserId,
    channel::Message,
};
use serenity::framework::standard::{
    Args,
    CommandGroup,
    CommandResult,
    StandardFramework,
    HelpOptions,
    help_commands::plain,
    macros::{command, group, help, hook}
};

use std::env;
use std::collections::HashSet;

mod char;
mod moves;
mod parameters;
use parameters::{Parameters, Modifier};

#[group]
#[commands(mv, moves, roll, char)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP)
        .help(&MY_HELP)
        .unrecognised_command(unrecognised_command);

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

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = plain(context, msg, args, &help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn unrecognised_command(ctx: &Context, msg: &Message, name: &str) {
    msg.reply(ctx, format!("Unknown command: {}", name)).await.unwrap();
}

#[command]
#[aliases(r)]
#[usage("[+Modifier] [hope/despair] [=Bring your own dice] [@Character]")]
#[example("+Grace,hope")]
#[example("-2")]
#[example("+2 d")]
/// Roll the dice with a given modifier, possibly with advantage or disadvantage
async fn roll(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
       let contents = match msg.content.find(' ') {
           Some(idx) => msg.content.split_at(idx + 1).1,
           None => "",
        };
        let param = parameters::parameters(contents)?;
        let (_, message) = calculate_roll(msg.author.id.0, param)?;
        Ok(message)
    }).await
}

#[command]
/// List all possible moves
async fn moves(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |_msg| {
        Ok(moves::get_moves()?.join("\n"))
    }).await
}

#[command("move")]
#[aliases(m)]
#[usage("MoveName [+Modifier],[hope/despair],[=Bring your own dice],[@Character]")]
#[example("Overcome")]
#[example("TalkSense @AshenOne")]
#[example("FinishBlood -1,despair")]
#[example("GetAway =2")]
/// Make a move, possibly with advantage or disadvantage
async fn mv(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
        let mut words = msg.content.split(' ');
        words.next(); // strip leading
        let move_name = words.next().context("Move name required")?;
        let mv = moves::get_move(move_name)?;
        let parameters = words.collect::<Vec<&str>>().join(" ");
        let mut param = parameters::parameters(&parameters)?;
        if let Some((ref stat, sign)) = mv.stat {
            param.modifiers.push(Modifier::Stat { stat, sign });
        }
        let (result, roll_text) = calculate_roll(msg.author.id.0, param)?;
        Ok(moves::get_move_text(mv, roll_text, result)?)
    }).await
}

#[command]
#[aliases(c)]
#[sub_commands(new, choose, stat)]
async fn char(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
        Ok(format!("You are set up to roll as {}", char::get_current_char(msg.author.id.0)?))
    }).await
}

#[command]
#[usage("YourCharacterName")]
/// Create a new character with an empty set of stats. Name must be one word and unique
async fn new(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
        let mut words = msg.content.split(' ');
        words.next();
        words.next(); // strip leading
        let name = words.next().context("No name provided")?;
        char::new(name)?;
        Ok(format!("Character with name {} created", name))
    }).await
}

#[command]
#[usage("YourCharacterName")]
/// Set a specific character to default to when rolling or making moves
async fn choose(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
        let mut words = msg.content.split(' ');
        words.next();
        words.next(); // strip leading
        let name = words.next().context("No name provided")?;
        char::choose(msg.author.id.0, name)?;
        Ok(format!("Set up to roll as {}", name))
    }).await
}

#[command]
#[usage("YourCharacterName YourStat YourValue")]
/// Set a given stat for your character
async fn stat(ctx: &Context, msg: &Message) -> CommandResult {
    command_wrapper(ctx, msg, |msg| {
        let mut words = msg.content.split(' ');
        words.next();
        words.next(); // strip leading
        let name = words.next().context("No name provided")?;
        let stat = words.next().context("No stat provided")?;
        let val = words.next().context("No value provided")?.parse::<i32>()?;
        char::set_stat(name, stat, val)?;
        Ok(format!("{}'s {} stat is now {}", name, stat, val))
    }).await
}

async fn command_wrapper(ctx: &Context, msg: &Message, f: impl FnOnce(&Message) -> Result<String>) -> CommandResult {
    let result = f(msg);
    match result {
        Ok(ok) => {
            msg.reply(ctx, ok).await?;
            Ok(())
        }
        Err(err) => {
            msg.reply(ctx, format!("**Error:** {}", err)).await?;
            Ok(())
        }
    }
}

fn calculate_roll(user: u64, Parameters { modifiers, character, hope, despair }: Parameters) -> Result<(i32, String)> {
    use std::cmp::{max, min};

    if hope && despair {
        // TODO: wording
        bail!("You can't roll with advantage AND disadvantage!");
    }
    
    let (mut mod_num, mut mod_string) = (0, String::new());
    let mut do_roll = true;
    for modifier in modifiers {
        match modifier {
            Modifier::Number(num) => {
                mod_num += num;
                mod_string.push_str(&format!("+{}", num));
            }
            Modifier::Stat { sign, stat } => {
                let value = sign * char::get_stat(character, user, stat)?;
                let sign = if sign > 0 { "+" } else { "-" };
                mod_num += value;
                mod_string.push_str(&format!("{}{} [{}]", sign, stat, value));
            }
            Modifier::Set(num) => {
                mod_num = num;
                mod_string = format!("manually input {}", mod_num);
                do_roll = false;
            }
        }
    }

    let mut sum = mod_num;
    let mut dice = String::new();

    if do_roll {
        let d1 = i32(1..6);
        let d2 = i32(1..6);

        sum += d1 + d2;

        if hope {
            let d3 = i32(1..6);
            let least = min(d1, min(d2, d3));
            sum += d3 - least;
            dice = format!("(Rolled {}, {} and {}; dropped {})", d1, d2, d3, least);
        } else if despair {
            let d3 = i32(1..6);
            let greatest = max(d1, max(d2, d3));
            sum += d3 - greatest;
            dice = format!("(Rolled {}, {} and {}; dropped {})", d1, d2, d3, greatest);
        } else {
            dice = format!("(Rolled {} and {})", d1, d2);
        }
    }

    let name = character
        .map(|c| Ok(c.to_owned()))
        .unwrap_or_else(|| char::get_current_char(user))?;

    Ok((sum, format!("{} got a **{}**, {}{}", name, sum, dice, mod_string)))
}
