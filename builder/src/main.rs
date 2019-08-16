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

#![allow(dead_code, unused)]
#![feature(decl_macro, proc_macro_hygiene, try_trait)]

#[allow(unused_imports)]
use crate::prelude::*;

use common::config::Config;
use ring::{digest, hmac};
use rocket::{post, routes};

mod payload;
mod prelude;
mod push;
mod result;

#[post("/", data = "<payload>")]
fn endpoint(payload: WebhookPayload) -> String {
    match payload {
        WebhookPayload::Push(payload) => push::handle(payload),
    };

    "Received".into()
}

pub struct WebhookKey(hmac::VerificationKey);

impl WebhookKey {
    pub fn new(config: &Config) -> Self {
        Self(hmac::VerificationKey::new(
            &digest::SHA1,
            config.github.webhook.secret.as_bytes(),
        ))
    }
}

fn main() {
    common::init();

    let config = Config::new();
    let webhook_key = WebhookKey::new(&config);
    let server = rocket::custom(config.as_rocket_config(15003))
        .mount("/", routes![endpoint,])
        .manage(config)
        .manage(webhook_key);
    info!("Starting builder server");
    let err = server.launch();
    panic!("{}", err);
}
