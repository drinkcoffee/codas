use eyre::{Context, Result};
use lib::prelude::*;
use serde::Deserialize;
use std::{fs, path::Path, str::FromStr};

#[derive(Deserialize)]
struct RawConfig {
    url_one: String,
    url_two: String,
    token_one: String,
    token_two: String,
}

#[derive(Debug)]
pub struct Config {
    url_one: Url,
    url_two: Url,
    token_address_one: Address,
    token_address_two: Address,
}

impl Config {
    pub fn parse(file_name: impl AsRef<Path>) -> Result<Config> {
        let file_str =
            fs::read_to_string(file_name).wrap_err("failed to open file at specified path")?;
        let raw =
            toml::from_str::<RawConfig>(&file_str).wrap_err("failed to parse config from toml")?;

        let url_one =
            Url::parse(&raw.url_one).wrap_err(format!("failed to parse url: {}", raw.url_one))?;
        let url_two =
            Url::parse(&raw.url_two).wrap_err(format!("failed to parse url: {}", raw.url_two))?;
        let token_address_one = Address::from_str(&raw.token_one)
            .wrap_err(format!("failed to parse address: {}", raw.token_one))?;
        let token_address_two = Address::from_str(&raw.token_two)
            .wrap_err(format!("failed to parse address: {}", raw.token_two))?;

        Ok(Self {
            url_one,
            url_two,
            token_address_one,
            token_address_two,
        })
    }
}