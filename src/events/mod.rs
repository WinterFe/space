mod autoban;
mod status;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, prelude::Ready},
};

use crate::events::{autoban::auto_ban_users, status::loop_status_update};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is online", data_about_bot.user.tag());

        loop_status_update(ctx).await;
    }

//     async fn message(&self, ctx: Context, new_message: Message) {
//         auto_ban_users(ctx, new_message).await;
//     }
}
