use serenity::{client::Context, model::channel::Channel};

use crate::SpaceResult;

pub async fn get_channel_from_id(ctx: &Context, id: u64) -> SpaceResult<Channel> {
    match ctx.cache.channel(id).await {
        Some(ch) => Ok(ch),
        None => Ok(ctx.http.get_channel(id).await?),
    }
}