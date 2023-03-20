use std::collections::HashMap;

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

pub type ServerConfigMap = HashMap<String, ServerConfig>;

#[derive(Debug, PartialEq)]
pub struct AlikConfig {
    pub discord: DiscordConfig,
    pub servers: ServerConfigMap,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::config::{parse_config, AlikConfig, DiscordConfig, ServerConfig};

    #[test]
    fn parses_single_server_config() {
        let config_string = String::from(
            "[discord]
            Token = X
            ApplicationID = 1
            GuildId = 2
            
            [training]
            StartBatchFilePath = foo
            StopBatchFilePath = bar
            ",
        );
        let result = parse_config(config_string);
        assert_eq!(
            result,
            Ok(AlikConfig {
                discord: DiscordConfig {
                    discord_token: "X".to_string(),
                    application_id: 1,
                    guild_id: 2,
                },
                servers: HashMap::from([(
                    "training".to_string(),
                    ServerConfig {
                        start_batch_file_path: "foo".to_string(),
                        stop_batch_file_path: "bar".to_string(),
                    }
                )]),
            })
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

const DISCORD_CONFIG_SECTION: &str = "discord";

fn parse_server_config(config: &Ini, section: &String) -> Result<ServerConfig, String> {
    let start_batch_file_path = config
        .get(&section, "StartBatchFilePath")
        .ok_or("StartBatchFilePath not found")?;

    let stop_batch_file_path = config
        .get(&section, "StopBatchFilePath")
        .ok_or("StopBatchFilePath not found")?;

    Ok(ServerConfig {
        start_batch_file_path,
        stop_batch_file_path,
    })
}

fn parse_server_configs(config: &Ini) -> Result<ServerConfigMap, String> {
    config
        .sections()
        .into_iter()
        .filter(|section| section != DISCORD_CONFIG_SECTION)
        .map(|section| parse_server_config(&config, &section).map(|sc| (section, sc)))
        .collect()
}

fn parse_discord_config(config: &Ini) -> Result<DiscordConfig, String> {
    let discord_token = config
        .get(DISCORD_CONFIG_SECTION, "Token")
        .ok_or("Token not found")?;

    let application_id_str = config
        .get(DISCORD_CONFIG_SECTION, "ApplicationID")
        .ok_or("ApplicationID not found")?;
    let application_id = application_id_str
        .parse()
        .map_err(|_| "ApplicationID is not a number")?;

    let guild_id_str = config
        .get(DISCORD_CONFIG_SECTION, "GuildID")
        .ok_or("GuildID not found")?;
    let guild_id = guild_id_str
        .parse()
        .map_err(|_| "GuildID is not a number")?;

    Ok(DiscordConfig {
        discord_token,
        application_id,
        guild_id,
    })
}

pub fn parse_config(config_string: String) -> Result<AlikConfig, String> {
    let mut config = Ini::new();
    config.read(config_string)?;

    Ok(AlikConfig {
        discord: parse_discord_config(&config)?,
        servers: parse_server_configs(&config)?,
    })
}
