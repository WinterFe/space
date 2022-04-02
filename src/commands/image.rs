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

use crate::apis::{get_nekoslife_api};
use crate::utils::constants::colors;

#[group]
#[commands(cat)]
#[description("Image 🖼️ - Commands related to image reponse(s), such as cats and dogs :D")]
pub struct Image;

#[command("cat")]
#[description("Cat pics!")]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    let api = get_nekoslife_api();
    let image = api.get_cat().await?;

    let mut embed = CreateEmbed::default();
    embed.title("Meow :3");
    embed.description(format!("[Cat!]({})", image.url));
    embed.image(image.url);
    embed.color(colors::PINK);

    msg.channel_id
        .send_message(ctx, |x| x.set_embed(embed).reference_message(msg))
        .await?;

    Ok(())
}
