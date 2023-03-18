use configparser::ini::Ini;

#[derive(Debug, PartialEq)]
pub struct DiscordConfig {
    pub discord_token: String,
    pub application_id: u64,
    pub guild_id: u64,
}

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub start_batch_file_path: String,
    pub stop_batch_file_path: String,
}

#[cfg(test)]
mod tests {
    use crate::config::{parse_config, DiscordConfig, ServerConfig};

    #[test]
    fn parses_config() {
        let config_string = String::from(
            "[discord]
            Token = X
            ApplicationID = 1
            GuildId = 2
            
            [server]
            StartBatchFilePath = foo
            StopBatchFilePath = bar
            ",
        );
        let result = parse_config(config_string);
        assert_eq!(
            result,
            Ok((
                DiscordConfig {
                    discord_token: "X".to_string(),
                    application_id: 1,
                    guild_id: 2,
                },
                ServerConfig {
                    start_batch_file_path: "foo".to_string(),
                    stop_batch_file_path: "bar".to_string(),
                },
            ))
        )
    }

    #[test]
    #[should_panic(expected = "Token not found")]
    fn fails_if_discord_token_missing() {
        let config_string = String::from(
            "[discord]
            ApplicationID = 5",
        );
        parse_config(config_string).unwrap();
    }

    #[test]
    #[should_panic(expected = "ApplicationID not found")]
    fn fails_if_application_id_missing() {
        let config_string = String::from(
            "[discord]
            Token = X",
        );
        parse_config(config_string).unwrap();
    }

    #[test]
    #[should_panic(expected = "ApplicationID is not a number")]
    fn fails_if_application_id_not_number() {
        let config_string = String::from(
            "[discord]
            Token = X
            ApplicationID = X",
        );
        parse_config(config_string).unwrap();
    }
}

pub fn parse_config(config_string: String) -> Result<(DiscordConfig, ServerConfig), String> {
    let mut config = Ini::new();
    config.read(config_string)?;

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
