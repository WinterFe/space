mod image;
mod moderation;
mod util;
use rand::{thread_rng, Rng};
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    framework::{
        standard::{
            macros::{help, hook},
            Args, CommandGroup, CommandResult, DispatchError, HelpOptions,
        },
        StandardFramework,
    },
    model::{channel::Message, id::UserId, user::User},
};
use std::collections::HashSet;
use tokio::spawn;

use crate::{
    components::embed::{Embed, IsEmbed},
    config::SpaceConfig,
    database::{
        functions::{
            bans::{create_ban_event, set_banned},
            custom_reaction::get_custom_reaction,
            guild::{get_db_guild, register_guild},
            user::register_user,
        },
        models::guild::DbGuildType,
    },
    errors::error_permission,
    utils::constants::{colors, msg_emojis},
    utils::user::get_user_from_id,
};

use chrono::{DateTime, TimeZone};
use chrono::{Local, Utc};

pub trait SystemTime
where
    Self: TimeZone,
{
    /// Construct a time from a timezone
    fn now() -> DateTime<Self>;
}

impl SystemTime for Utc {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }
}

impl SystemTime for Local {
    fn now() -> DateTime<Local> {
        Local::now()
    }
}

pub fn create_framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|x| {
            x.dynamic_prefix(move |ctx, msg| {
                Box::pin(async move {
                    if let Some(guild) = msg.guild(ctx).await {
                        if let Ok(db_guild) = get_db_guild(guild).await {
                            return Some(db_guild.prefix);
                        }
                    }
                    Some(SpaceConfig::get_default_prefix())
                })
            })
            .prefix("s!")
            .on_mention(Some(SpaceConfig::get_id_mention()))
            .no_dm_prefix(true)
            .case_insensitivity(true)
            .owners(vec![UserId(683530527239962627)].into_iter().collect())
        })
        .group(&util::UTIL_GROUP)
        // .group(&weeb::WEEB_GROUP)
        // .group(&config::CONFIG_GROUP)
        .group(&moderation::MODERATION_GROUP)
        .group(&image::IMAGE_GROUP)
        // .group(&nsfw::NSFW_GROUP)
        // .group(&about::ABOUT_GROUP)
        // .group(&owner::OWNER_GROUP)
        // .group(&custom_reaction::CUSTOMREACTION_GROUP)
        .before(before_command)
        .after(after_command)
        .normal_message(normal_message)
        .on_dispatch_error(dispatch_error)
        .help(&HELP)
}

#[help]
async fn help(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    _: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    _: HashSet<UserId>,
) -> CommandResult {
    if args.is_empty() {
        let images = vec![
            "https://lunardev.group/media/space/1.gif",
            "https://lunardev.group/media/space/2.gif",
            "https://lunardev.group/media/space/3.gif",
        ];

        let random = thread_rng().gen_range(0..images.len());

        let mut embed = CreateEmbed::default();
        embed.title("Available Commands");
        embed.description("To get more info on a command, type `help {command}`");
        embed.image(images[random]);
        embed.color(colors::PINK);

        for group in groups.iter() {
            if !group.options.help_available {
                continue;
            }

            let group_description = group.options.description.unwrap_or(group.name);
            let group_cmds = group.options.commands;

            let mut group_cmds_name = "".to_string();
            for cmd in group_cmds.iter() {
                if cmd.options.help_available {
                    group_cmds_name
                        .push_str(format!(" `{}`", cmd.options.names.first().unwrap()).as_str());
                }
            }

            embed.field(group_description, group_cmds_name, false);
        }

        msg.channel_id
            .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
            .await?;
    } else {
        let cmd_name = args.single::<String>()?;

        let prefix = if let Some(guild) = msg.guild(ctx).await {
            if let Ok(db_guild) = get_db_guild(guild).await {
                db_guild.prefix
            } else {
                SpaceConfig::get_default_prefix()
            }
        } else {
            "".to_string()
        };

        let mut embed = CreateEmbed::default();
        embed.color(colors::PURPLE);

        let images = vec![
            "https://lunardev.group/media/space/1.gif",
            "https://lunardev.group/media/space/2.gif",
            "https://lunardev.group/media/space/3.gif",
        ];

        let random = thread_rng().gen_range(0..images.len());

        if cmd_name == "help" {
            embed.image(images[random]);
            embed.title("More info for help");
            embed.description("The help command provides a list of all usable commands.");
            embed.field("Use", format!("`{0}help <command>*`", prefix), false);
            embed.field(
                "Examples",
                format!("`{0}help prefix`\n`{0}help`", prefix),
                false,
            );
        } else {
            let mut cmd = None;

            for group in groups.iter() {
                for cmds in group.options.commands.iter() {
                    if cmds.options.names.iter().any(|x| x == &cmd_name) {
                        cmd = Some(cmds);
                    }
                }
            }

            match cmd {
                None => {
                    embed.title("[CMDS] Command not found");
                }
                Some(cmd) => {
                    if !cmd.options.help_available {
                        let db_guild = if let Some(guild) = msg.guild(ctx).await {
                            if let Ok(guild) = get_db_guild(guild).await {
                                Some(guild)
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        match db_guild {
                            Some(db_guild) => {
                                if db_guild.guild_type == DbGuildType::Normal as u32 {
                                    return Ok(());
                                }
                            }
                            None => {
                                return Ok(());
                            }
                        }
                    }

                    let images = vec![
                        "https://lunardev.group/media/space/1.gif",
                        "https://lunardev.group/media/space/2.gif",
                        "https://lunardev.group/media/space/3.gif",
                    ];

                    let random = thread_rng().gen_range(0..images.len());

                    embed.image(images[random]);
                    embed.title(format!("Command: {}", cmd.options.names[0]));
                    let mut footer = CreateEmbedFooter::default();
                    footer.text("<> is required, [] is optional");
                    embed.set_footer(footer);
                    if let Some(description) = cmd.options.desc {
                        embed.description(description);
                    }
                    if cmd.options.names.len() > 1 {
                        let aliases = cmd
                            .options
                            .names
                            .iter()
                            .skip(1)
                            .fold("".to_string(), |result, item| {
                                format!("{}\n- {}", result, item)
                            });
                        embed.field("Aliases", aliases, false);
                    }
                    if let Some(usage) = cmd.options.usage {
                        embed.field("Use", format!("`{}{}`", prefix, usage), false);
                    }
                    if !cmd.options.examples.is_empty() {
                        let examples = cmd
                            .options
                            .examples
                            .iter()
                            .fold("".to_string(), |result, item| {
                                format!("{}\n`{}{}`", result, prefix, item)
                            });
                        embed.field(
                            if cmd.options.examples.len() > 1 {
                                "Examples"
                            } else {
                                "Example"
                            },
                            examples,
                            false,
                        );
                    }
                }
            };
        }

        msg.channel_id
            .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
            .await?;
    }

    Ok(())
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, err: DispatchError) {
    match err {
        DispatchError::LackingPermissions(permissions) => {
            error_permission(ctx, msg, permissions).await
        }
        _ => return,
    }
}

#[hook]
async fn normal_message(ctx: &Context, msg: &Message) {
    if let Some(guild) = msg.guild(ctx).await {
        let content = &msg.content;
        let gcheck = msg.guild_id.unwrap().to_guild_cached(ctx).await.unwrap();

        // let bans = gcheck.bans(ctx).await;
        // println!("{:?}", bans);

        // Check if user is in the database, if not, add them :) (Ignoring bots, obviously)
        if msg.author.bot {
            return;
        } else {
            let uid: i64 = msg.author.id.to_string().parse().unwrap();
            let mut _registered = register_user(
                uid,
                msg.author.name.to_string(),
                msg.author.name.to_string(),
            )
            .await;
        }

        if msg.mentions_user_id(683530527239962627 | 959858563503759441 | 830647018736058368) {
            let banned_at = chrono::Utc::now().naive_utc().to_string();
            let uid: i64 = msg.author.id.to_string().parse().unwrap();
            let user_id: u64 = msg.author.id.to_string().parse().unwrap();

            let user = get_user_from_id(ctx, user_id).await.unwrap();
            let guild = msg.guild_id.unwrap().to_guild_cached(ctx).await.unwrap();

            if msg.author.id == 000 {
                return;
            } else {
                match guild
                    .ban_with_reason(
                        ctx,
                        &user,
                        0,
                        "[AUTO] Pinging an owner/co-owner. | 1 week ban",
                    )
                    .await
                {
                    Ok(_) => {
                        let mut embed = CreateEmbed::default();
                        embed.description(format!(
                            "**{}** has been banned!\nReason: {}",
                            user.tag(),
                            "[Auto] Pinging an owner/co-owner."
                        ));
                        embed.color(colors::PURPLE);
                        let _ = msg
                            .channel_id
                            .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
                            .await;

                        let mut _set_banned = set_banned(uid, "true", banned_at.as_str()).await;
                        let mut _set_ban_event =
                            create_ban_event(uid, format!("BAN_{}", uid.to_string())).await;
                        send_alert(ctx, msg, &msg.author, "Banned", &guild.name, "\"Pinging the Co-Owners Or the Owner will result in a temp ban for 1 week.\"").await;
                    }
                    Err(error) => {
                        println!("Oh noes: {}", error);
                    }
                }
            }
        }
        match get_custom_reaction(guild, content).await {
            Ok(Some(cr)) => {
                let is_embed = Embed::from_str(ctx, msg, &cr.reply).await;
                match is_embed {
                    IsEmbed::Embed(embed, result) => {
                        let msg_send = msg
                            .channel_id
                            .send_message(ctx, move |x| {
                                if let Some(text) = &embed.plain_text {
                                    x.content(text);
                                }
                                x.set_embed(embed.into())
                            })
                            .await;

                        if msg_send.is_err() {
                            msg.channel_id
                                .send_message(ctx, |x| x.content(result))
                                .await
                                .ok();
                        }
                    }
                    IsEmbed::Message(text) => {
                        msg.channel_id
                            .send_message(ctx, |x| x.content(text))
                            .await
                            .ok();
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err)
            }
            _ => {}
        }
    }
}

#[hook]
async fn before_command(ctx: &Context, msg: &Message, name: &str) -> bool {
    match msg.guild_id {
        Some(guild_id) => {
            let guild = guild_id.to_guild_cached(ctx).await;
            let thread = spawn(async move {
                if let Some(guild) = guild {
                    register_guild(guild).await
                } else {
                    Err("[CMDS] Failed to get guild.".into())
                }
            });

            if name == "prefix" || name == "Space" {
                return match thread.await {
                    Ok(result) => result.is_ok(),
                    Err(_) => false,
                };
            }

            true
        }
        None => true,
    }
}

#[hook]
async fn after_command(ctx: &Context, msg: &Message, name: &str, why: CommandResult) {
    let date = chrono::Local::now();
    let divider = "••• ";

    if let Err(why) = why {
        println!(
            "\nCommand Errored:\n{}Time: {}\n{}User: {}\n{}Command: {}\n{}Error: {:#?}",
            divider,
            date.format("%Y-%m-%d [%H:%M:%S]"),
            divider,
            msg.author.tag(),
            divider,
            name,
            divider,
            why
        );

        let mut embed = CreateEmbed::default();
        embed.title("Error!");
        embed.description(format!(
            "{} Error running the command `{}`:\n```{}```\n> Please report this error to: `{}`",
            msg_emojis::TICKNO,
            name,
            why,
            "AceFifi#2000"
        ));
        embed.color(colors::RED);

        // let _ = msg.react(ctx, '❌').await;
        let _ = msg
            .channel_id
            .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
            .await;
    } else {
        println!(
            "\nCommand Processed:\n{}Time: {}\n{}User: {}\n{}Command: {}\n",
            divider,
            date.format("%Y-%m-%d [%H:%M:%S]"),
            divider,
            msg.author.tag(),
            divider,
            name
        );
    }
}

async fn send_alert(
    ctx: &Context,
    _msg: &Message,
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
    embed.field("By: ", "Automated Action", false);

    let dm = user.create_dm_channel(ctx).await;

    if dm.is_ok() {
        let dm = dm.unwrap();
        dm.send_message(ctx, |x| x.set_embed(embed)).await.ok();
    }
}
