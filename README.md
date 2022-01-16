# PBotA

A Discord Bot for playing some PbtA games!

My friends and I were playing [Fellowship 2e](TODO) and I wanted to set up a Discord bot to handle die rolling. Of course, you can just roll 2d6 + some modifier on any dice bot of your choice, but I thought that "why not remove the friction and just teach the bot our character's stats?" Then, I thought "why have us roll +Grace and then check the text of Get Away when I can teach the bot the move results?" So I added some facilities for defining moves (following this pattern I also wrote a script to make defining moves more convenient), and that's where we are today.

You should be able to take any game that rolls 2d6 and adds a stat, and has specific moves tied to specific stats with specific, ranged outcomes. PbtA games that don't follow this formula from the "Apocalypse Engine" (such as *Blades in the Dark* or *Mobile Frame Zero: Firebrands*) aren't supported.

## Setting up the bot

A word of warning: I'm not going to host this bot for you. What you're getting here is some free source code, and nothing more than that. To set this up you'll need to be comfortable rolling up your sleeves and poking around at a few computer things.

### Adding the bot to your server

In these steps you'll create a Discord bot account and add it to your server.

1. First, you'll need to [set up your bot on Discord itself](https://discord.com/developers/docs/intro#bots-and-apps). I think you should just be able to go to [the developer portal](https://discord.com/developers/applications) for this but I don't remember if there's more setup involved.
2. Create a new application and give it a name. You can call it whatever you want (including PBotA).
3. Go to the "Bot" section of your application and create a bot user
4. Use the permissions dialog to create a permissions integer. You'll need "Read Messages & View Channels" under General and "Send Messages" under Messages. That should be 3072, but it pays to double-check yourself.
5. Copy your bot's ApplicationID from "General Information" and the permissions integer from step #4 and replace APPLICATIONID and PERMISSIONS in the following url: https://discord.com/api/oauth2/authorize?client_id=APPLICATIONID&permissions=PERMISSIONS&scope=bot%20applications.commands. It should look something like https://discord.com/api/oauth2/authorize?client_id=12345678910&permissions=3072&scope=bot%20applications.commands
6. You should be able to add your new bot user to your Discord server!

### Running the bot

Now that you have the bot account on your server, it's time to power it up.

1. You'll need a recent copy of the Rust compiler. You can get one for your operating system from [rustup](https://rustup.rs/).
2. Once you have the Rust compiler installed, download the source code for PBotA. You can either clone it via Git, or if those words mean nothing to you, select "Download Zip" under the "Code" section on Github.
3. Using the command line, navigate to the folder where you have PBotA downloaded and run `cargo build -p pbota-bot`. If everything builds succesfully, it's on to the next step.
4. Find your bot's client token under "Bot" on the Discord settings page.
5. Set this token as an environment variable in the command line called "DISCORD_TOKEN." On Windows you should be able to do this via the command `set DISCORD_TOKEN=MY_TOKEN`; on macOS and Linux, you'll want `DISCORD_TOKEN=MY_TOKEN`. (Yes I know if you're using Powershell or Fish or what have you the command will be different. I'm sure you can figure it out in that case).
6. Run the command `cargo run -p pbota-bot`.

If you've added the bot to your server, you have the correct token, and the project runs and builds without errors, you should be good to go! Try saying `-help` in any channel on your Discord server to test the bot.
