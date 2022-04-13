#[allow(unused_imports)]
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
        Args
    },
    model::{channel::Message, user::User},
};

use crate::utils::{
    constants::colors,
    user::{get_user_from_args, get_user_role_position},
};

#[group]
#[commands(kick)]
#[description("Mod 🛠️ - Administration/Moderation commands")]
pub struct Moderation;

#[command("kick")]
#[required_permissions("KICK_MEMBERS")]
#[min_args(1)]
#[description("Kicks a user from the server")]
#[usage("kick <user> [reason]")]
#[example("kick @AceFifi")]
#[example("kick @AceFifi being annoying")]
#[example("kick 683530527239962627")]
#[example("kick 683530527239962627 being annoying")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = get_user_from_args(ctx, &mut args).await?;

    let reason = args.remains().unwrap_or("No Reason");
    let guild = msg.guild_id.unwrap().to_guild_cached(ctx).await.unwrap();

    let member_role = get_user_role_position(ctx, &guild, &user).await?;

    let author_role = get_user_role_position(ctx, &guild, &msg.author).await?;

    let bot_role =
        get_user_role_position(ctx, &guild, &ctx.cache.current_user().await.into()).await?;


    if author_role > member_role && bot_role > member_role {
        send_alert(ctx, msg, &user, "kicked", &guild.name, reason).await;
        match guild.kick_with_reason(ctx, &user.id, reason).await {
            Ok(_) => {
                let mut embed = CreateEmbed::default();
                embed.description(format!(
                    "{} has been kicked!",
                    user.tag()
                ));
                embed.color(colors::PURPLE);

                msg.channel_id
                    .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
                    .await?;
            }
            Err(error) => { println!("Oh noes: {}", error); }
        };
    } else {
        let mut embed = CreateEmbed::default();
        embed.title("Error");
        embed.description("Unable to kick member, insufficient permissions.");
        embed.color(colors::YELLOW);

        msg.channel_id
            .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
            .await?;
    }

    Ok(())
}

async fn send_alert(
    ctx: &Context,
    msg: &Message,
    user: &User,
    tpe: &str,
    guild_name: &str,
    reason: &str,
) {
    let mut embed = CreateEmbed::default();
    embed.title("**Attention**");
    embed.description(format!("You were {} from **{}**", tpe, guild_name));
    embed.color(colors::PURPLE);
    embed.field("Reason: ", reason, false);
    embed.field("By: ", msg.author.tag(), false);

    let dm = user.create_dm_channel(ctx).await;

    if dm.is_ok() {
        let dm = dm.unwrap();
        dm.send_message(ctx, |x| x.set_embed(embed)).await.ok();
    }
}
