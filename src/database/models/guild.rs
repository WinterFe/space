pub struct DbGuild {
    pub discord_id: u64,
    pub name: String,
    pub prefix: String,
    pub guild_type: u32,
}

pub enum DbGuildType {
    Normal = 0,
    Vip = 1,
    Owner = 2,
}

impl From<u32> for DbGuildType {
    fn from(el: u32) -> Self {
        match el {
            1 => DbGuildType::Vip,
            2 => DbGuildType::Owner,
            _ => DbGuildType::Normal,
        }
    }
}

impl From<DbGuildType> for u32 {
    fn from(el: DbGuildType) -> Self {
        match el {
            DbGuildType::Normal => 0,
            DbGuildType::Vip => 1,
            DbGuildType::Owner => 2,
        }
    }
}