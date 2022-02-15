use std::env;

pub struct DiscordConfig {
    pub discord_token: String,
    pub application_id: u64,
}

pub struct BotConfig {
    pub guild_id: u64,
    pub start_batch_file_path: String,
}

pub fn parse_config() -> Result<(DiscordConfig, BotConfig), String> {
    let discord_token =
        env::var("DISCORD_TOKEN").map_err(|_| "DISCORD_TOKEN env variable not found")?;

    let application_id_str =
        env::var("APPLICATION_ID").map_err(|_| "APPLICATION_ID env variable not found")?;
    let application_id = application_id_str
        .parse()
        .map_err(|_| "APPLICATION_ID is not a valid id")?;

    let guild_id_str = env::var("GUILD_ID").map_err(|_| "GUILD_ID env variable not found")?;
    let guild_id = guild_id_str
        .parse()
        .map_err(|_| "GUILD_ID is not a valid number")?;

    let start_batch_file_path = env::var("START_BATCH_FILE_PATH")
        .map_err(|_| "START_BATCH_FILE_PATH env variable not found")?;

    Ok((DiscordConfig {
        discord_token,
        application_id,
    }, BotConfig {
        guild_id,
        start_batch_file_path,
    }))
}
