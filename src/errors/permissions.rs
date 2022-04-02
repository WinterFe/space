use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter},
    client::Context,
    model::{
        channel::{Channel, Message},
        Permissions,
    },
};

use crate::utils::constants::{colors, emojis};

pub async fn error_permission(ctx: &Context, msg: &Message, permissions: Permissions) {
    if let Some(guild) = &msg.guild(ctx).await {
        if let (Ok(member), Some(Channel::Guild(channel))) =
            (&msg.member(ctx).await, &msg.channel(ctx).await)
        {
            if let Ok(user_perm) = guild.user_permissions_in(channel, member) {
                let has_perm = user_perm & permissions;
                let has_not_perm = !has_perm & permissions;

                let match_names = |perms: Vec<&str>| {
                    perms
                        .iter()
                        .map(|perm| match *perm {
                            "Add Reactions" => "Add Reactions",
                            "Administrator" => "Administrator",
                            "Attach Files" => "Attach Files",
                            "Ban Members" => "Ban Members",
                            "Change Nickname" => "Change Nickname",
                            "Connect" => "Connect",
                            "Create Invite" => "Create Invite",
                            "Deafen Members" => "Deafen Members",
                            "Embed Links" => "Embed Links",
                            "Use External Emojis" => "Use External Emojis",
                            "Kick Members" => "Kick Memberes",
                            "Manage Channels" => "Manage Channels",
                            "Manage Emojis" => "Manage Emojis",
                            "Manage Guilds" => "Manage Guilds",
                            "Manage Messages" => "Manage Messages",
                            "Manage Nicknames" => "Manage Nicjnames",
                            "Manage Roles" => "Manage Roles",
                            "Manage Webhooks" => "Manage Webhooks",
                            "Mention Everyone" => "Mention @everyone",
                            "Move Members" => "Move Members",
                            "Mute Members" => "Mute Members",
                            "Priority Speaker" => "Priority Speaker",
                            "Read Message History" => "Read Message History",
                            "Request To Speak" => "Request To Speak",
                            "Read Messages" => "Read Messages",
                            "Send Messages" => "Send Messages",
                            "Send TTS Messages" => "Send TTS Messages",
                            "Speak" => "Speak",
                            "Stream" => "Stream",
                            "Use Slash Commands" => "Use Slash Commands",
                            "Use Voice Activity" => "Use Voice Activity",
                            "View Audit Log" => "View Audit Log",
                            _ => "Unknown Permission(s)",
                        })
                        .collect::<Vec<&str>>()
                };

                let has_perm = match_names(has_perm.get_permission_names());
                let has_not_perm = match_names(has_not_perm.get_permission_names());

                let has_perm = has_perm.iter().fold("".to_string(), |result, item| {
                    format!("{} <:{}> {}\n", result, emojis::ENABLED, item)
                });
                let has_not_perm = has_not_perm.iter().fold("".to_string(), |result, item| {
                    format!("{} <:{}> {}\n", result, emojis::DISABLED, item)
                });

                let perms = format!("{}{}", has_perm, has_not_perm);

                let mut embed = CreateEmbed::default();
                embed.title("Please check permissions");
                embed.description(perms);
                embed.color(colors::YELLOW);

                let mut footer = CreateEmbedFooter::default();
                footer.text("All must be active");
                embed.set_footer(footer);

                msg.channel_id
                    .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
                    .await
                    .ok();
            }
        }
    }
}