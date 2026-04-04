use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

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

#[derive(Deserialize)]
struct AzureTokenResponse {
    access_token: String,
    expires_in: u64,
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

    pub  async fn get_auth_header(&self) -> Result<String, reqwest::Error> {
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

                Ok(format!("Bearer {}", token))
            }
        }
    }
}

pub fn test() {
    println!("Hello, world!");
}
