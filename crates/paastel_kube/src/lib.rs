// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client};

// const SECRET_GROUP_NAME: &str = "paastel.io";

#[derive(Debug, thiserror::Error)]
pub enum Error {}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct UserSecret {
    pub username: String,
    pub password: String,
    pub secret_name: String,
}

pub type UserSecrets = Vec<UserSecret>;

#[async_trait]
pub trait Secrets {
    async fn list_user_secrets(&self) -> Result<UserSecrets>;
}

#[derive(Debug)]
pub struct KubeSecrets {
    secrets: Api<Secret>,
}

impl KubeSecrets {
    pub fn new(client: Client) -> Self {
        Self {
            secrets: Api::default_namespaced(client),
        }
    }
}

#[async_trait]
impl Secrets for KubeSecrets {
    async fn list_user_secrets(&self) -> Result<UserSecrets> {
        let lp = ListParams::default()
            .match_any()
            .timeout(60)
            .labels("kubernetes.io/lifecycle=spot");
        let _s = self.secrets.list(&lp).await;
        todo!()
    }
}