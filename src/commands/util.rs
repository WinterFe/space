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

use crate::database::{
    get_database_connection,
    functions::{guild::get_db_guild},
    models::guild::{DbGuild, DbGuildType}
};

#[group]
#[commands(ping, gtype)]
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

#[command("gtype")]
#[description("Checks the curent servers type.")]
async fn gtype(ctx: &Context, msg: &Message) -> CommandResult {
    let mut gtype = String::from("");
    
    let guild = msg.guild(ctx).await.unwrap();
    let db_guild = get_db_guild(guild).await?;

    if db_guild.guild_type == DbGuildType::Normal as u32 {
        gtype.push_str("Normal");
    } else if db_guild.guild_type == DbGuildType::Vip as u32 {
        gtype.push_str("VIP");
    } else if db_guild.guild_type == DbGuildType::Owner as u32 {
        gtype.push_str("Owner");
    } else {
        gtype.push_str("Unknown");
    }

    let ginfo = msg.guild(ctx).await.unwrap();

    let mut embed = CreateEmbed::default();
    embed.title("Guild Type:");
    embed.thumbnail(format!("https://cdn.discordapp.com/icons/{}/{:?}", ginfo.id, ginfo.icon));
    embed.description(format!("{}", gtype));
    embed.color(colors::PINK);

    msg.channel_id.send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
    .await?;

    Ok(())
}