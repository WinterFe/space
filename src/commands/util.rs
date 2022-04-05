#[allow(unused_imports)]
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

use crate::utils::{
    constants::{colors}
};

#[group]
#[commands(ping)]
#[description("Util 🛠️ - Essential/Utility Commands")]
pub struct Util;

#[command("ping")]
#[aliases("pong", "lat", "latency")]
#[description("Checks the bots latency/ping.")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.title("Pong!");
    // embed.thumbnail(format!("{:?}", msg.author.avatar_url()));
    embed.description("I'm alive >.>");
    embed.color(colors::PINK);

    msg.channel_id.send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
    .await?;

    Ok(())
}