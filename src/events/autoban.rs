use serenity::{client::Context, model::channel::Message};

pub async fn auto_ban_users(_ctx: Context, new_message: Message) {
    print!("{}", new_message.content);
}