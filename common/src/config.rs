// Poggit
// Copyright (C) Poggit
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affer General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub postgres: PostgresConfig,
    pub github: GithubConfig,
    pub http: HttpConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GithubConfig {
    pub app: GithubAppConfig,
    pub webhook: GithubWebhookConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GithubAppConfig {
    pub id: u32,
    pub slug: String,
    pub client: String,
    pub secret: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GithubWebhookConfig {
    pub secret: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct HttpConfig {
    pub env: HttpEnv,
    pub address: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub enum HttpEnv {
    Development,
    Staging,
    Production,
}

impl Config {
    pub fn new() -> Self {
        let mut merger = ::config::Config::new();
        merger
            .merge(::config::Environment::new().separator("_"))
            .expect("Config error");
        merger.try_into().expect("Config error")
    }

    pub fn as_rocket_config(&self, port: u16) -> rocket::Config {
        use rocket::config::Environment as Env;
        rocket::Config::build(match self.http.env {
            HttpEnv::Development => Env::Development,
            HttpEnv::Staging => Env::Staging,
            HttpEnv::Production => Env::Production,
        })
        .address(
            self.http
                .address
                .as_ref()
                .map_or("0.0.0.0", |string| string.as_str()),
        )
        .port(port)
        .workers(64)
        .secret_key(base64::encode(&rand::random::<[u8; 32]>()))
        .finalize()
        .expect("Config error")
    }
}
