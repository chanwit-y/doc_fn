use reqwest::{Client, Error, Method, RequestBuilder};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

fn u64_from_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU64 {
        String(String),
        U64(u64),
    }
    match StringOrU64::deserialize(deserializer)? {
        StringOrU64::String(s) => s.parse().map_err(serde::de::Error::custom),
        StringOrU64::U64(n) => Ok(n),
    }
}

#[derive(Clone)]
enum AuthMethod {
    ApiKey(String),
    AzureServicePrincipal {
        tenant_id: String,
        client_id: String,
        client_secret: String,
        scope: String,
    },
}

#[derive(Clone)]
struct TokenCache {
    token: String,
    expires_at: Instant,
}

#[derive(Deserialize, Debug)]
struct AzureTokenResponse {
    token_type: String,
    #[serde(deserialize_with = "u64_from_str")]
    expires_in: u64,
    #[serde(deserialize_with = "u64_from_str")]
    ext_expires_in: u64,
    #[serde(deserialize_with = "u64_from_str")]
    expires_on: u64,
    #[serde(deserialize_with = "u64_from_str")]
    not_before: u64,
    resource: String,
    access_token: String,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
    auth: AuthMethod,
    token_cache: Arc<RwLock<Option<TokenCache>>>,
}

impl ApiClient {
    pub fn with_api_key(base_url: &str, api_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            auth: AuthMethod::ApiKey(api_key.to_string()),
            token_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_azure_sp(
        base_url: &str,
        tenant_id: &str,
        client_id: &str,
        client_secret: &str,
        scope: &str,
    ) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            auth: AuthMethod::AzureServicePrincipal {
                tenant_id: tenant_id.to_string(),
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                scope: scope.to_string(),
            },
            token_cache: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn get_auth_header(&self) -> Result<String, reqwest::Error> {
        match &self.auth {
            AuthMethod::ApiKey(key) => Ok(format!("Bearer {}", key)),
            AuthMethod::AzureServicePrincipal {
                tenant_id,
                client_id,
                client_secret,
                scope,
            } => {
                {
                    let cache = self.token_cache.read().await;
                    if let Some(ref cached) = *cache {
                        if cached.expires_at > Instant::now() + Duration::from_secs(60) {
                            return Ok(format!("Bearer {}", cached.token));
                        }
                    }
                }

                let token_url = format!(
                    "https://login.microsoftonline.com/{}/oauth2/token",
                    tenant_id
                );

                let resp = self
                    .client
                    .post(&token_url)
                    .form(&[
                        ("grant_type", "client_credentials"),
                        ("client_id", client_id),
                        ("client_secret", client_secret),
                        ("scope", scope),
                    ])
                    .send()
                    .await?
                    .json::<AzureTokenResponse>()
                    .await?;

                let token = resp.access_token.clone();
                let expires_at = Instant::now() + Duration::from_secs(resp.expires_in);

                {
                    let mut cache = self.token_cache.write().await;
                    *cache = Some(TokenCache {
                        token: token.clone(),
                        expires_at,
                    });
                }

                Ok(format!("Bearer {}", token))
            }
        }
    }

    async fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        let auth = self.get_auth_header().await?;
        Ok(self
            .client
            .request(method, format!("{}{}", self.base_url, path))
            .header("Authorization", auth))
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, Error> {
        self.request(Method::GET, path)
            .await?
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::POST, path)
            .await?
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn put<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PUT, path)
            .await?
            .json(body)
            .send()
            .await?
            .json::<T>()
            .await
    }

    pub async fn delete<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, Error> {
        self.request(Method::DELETE, path)
            .await?
            .send()
            .await?
            .json::<T>()
            .await
    }
}
