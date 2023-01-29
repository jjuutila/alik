use configparser::ini::Ini;

pub struct DiscordConfig {
    pub discord_token: String,
    pub application_id: u64,
    pub guild_id: u64,
}

pub struct ServerConfig {
    pub start_batch_file_path: String,
    pub stop_batch_file_path: String,
}

pub fn parse_config() -> Result<(DiscordConfig, ServerConfig), String> {
    let mut config = Ini::new();
    config.load("config.ini")?;

    let discord_token = config.get("discord", "Token").ok_or("Token not found")?;

    let application_id_str = config
        .get("discord", "ApplicationID")
        .ok_or("ApplicationID not found")?;
    let application_id = application_id_str
        .parse()
        .map_err(|_| "ApplicationID is not a number")?;

    let guild_id_str = config
        .get("discord", "GuildID")
        .ok_or("GuildID not found")?;
    let guild_id = guild_id_str
        .parse()
        .map_err(|_| "GuildID is not a number")?;

    let start_batch_file_path = config
        .get("server", "StartBatchFilePath")
        .ok_or("StartBatchFilePath not found")?;

    let stop_batch_file_path = config
        .get("server", "StopBatchFilePath")
        .ok_or("StopBatchFilePath not found")?;

    Ok((
        DiscordConfig {
            discord_token,
            application_id,
            guild_id,
        },
        ServerConfig {
            start_batch_file_path,
            stop_batch_file_path,
        },
    ))
}
