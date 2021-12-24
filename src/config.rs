use std::env;

pub struct Config {
    pub discord_token: String,
    pub application_id: u64,
}

pub fn parse_config() -> Result<Config, String> {
    let discord_token =
        env::var("DISCORD_TOKEN").map_err(|_| "DISCORD_TOKEN env variable not found")?;
    let application_id_str =
        env::var("APPLICATION_ID").map_err(|_| "APPLICATION_ID env variable not found")?;

    let application_id = application_id_str
        .parse()
        .map_err(|_| "APPLICATION_ID is not a valid id")?;

    Ok(Config {
        discord_token,
        application_id,
    })
}
