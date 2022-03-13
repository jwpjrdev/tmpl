use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub path: Option<String>, // todo: implement
    pub commands: ConfigCommands,
}

// todo: Option<String> for each command; what if they're deleted from the config file?
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct ConfigCommands {
    pub pre_install: String,
    pub install: String,
    pub post_install: String,
}

pub(crate) fn parse(config: &str) -> Config {
    toml::from_str(config).unwrap()
}
