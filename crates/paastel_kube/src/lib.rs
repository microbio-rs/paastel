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

use std::{result::Result as StdResult, collections::HashMap};

use async_trait::async_trait;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, core::ObjectList, Api, Client, Error as KError};

use paastel::{AuthKubeSecretPort, AuthUsers, Error as PaastelError, Result, Username, AuthUser};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

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

    async fn get_all(&self) -> StdResult<ObjectList<Secret>, KError> {
        let lp = ListParams::default()
            .match_any()
            .timeout(60)
            .labels("kubernetes.io/lifecycle=spot");
        Ok(self.secrets.list(&lp).await?)
    }
}

#[async_trait]
impl AuthKubeSecretPort for KubeSecrets {
    async fn list(&self) -> Result<AuthUsers> {
        let secrets = self
            .get_all()
            .await
            .map_err(|e| PaastelError::KubePort(e.to_string()))?;

        let content: HashMap<<Username, AuthUser> = secrets.into_iter().map(|c| {
            let auth_user = //
            todo!()
            (c.username, auth_user)
        }).collect();

        let auth_users = AuthUsers::new(content);
        Ok(auth_users)
    }
}
