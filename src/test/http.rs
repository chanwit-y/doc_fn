use dotenvy::dotenv;
use crate::core::http::ApiClient;

#[tokio::test]
async fn test_azure_sp_auth() {
    dotenv().ok();

    let tenant_id = std::env::var("AAD_SP_TENANT_ID").expect("AAD_SP_TENANT_ID must be set");
    let client_id = std::env::var("AAD_SP_CLIENT_ID").expect("AAD_SP_CLIENT_ID must be set");
    let client_secret =
        std::env::var("AAD_SP_CLIENT_SECRET").expect("AAD_SP_CLIENT_SECRET must be set");
    let scope = std::env::var("AAD_SP_CLIENT_SCOPE").expect("AAD_SP_CLIENT_SCOPE must be set");

    let client = ApiClient::with_azure_sp(
        "https://graph.microsoft.com/v1.0",
        &tenant_id,
        &client_id,
        &client_secret,
        &scope,
    );

    let auth_header = client
        .get_auth_header()
        .await
        .expect("Failed to get auth header");

    assert!(
        auth_header.starts_with("Bearer "),
        "Auth header should start with 'Bearer '"
    );
}
