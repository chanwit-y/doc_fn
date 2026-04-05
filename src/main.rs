mod core;

use dotenvy::dotenv;
use core::http::ApiClient;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let tenant_id = std::env::var("AAD_SP_TENANT_ID").expect("AAD_SP_TENANT_ID must be set");
    let client_id = std::env::var("AAD_SP_CLIENT_ID").expect("AAD_SP_CLIENT_ID must be set");
    let client_secret = std::env::var("AAD_SP_CLIENT_SECRET").expect("AAD_SP_CLIENT_SECRET must be set");
    let scope = std::env::var("AAD_SP_CLIENT_SCOPE").expect("AAD_SP_CLIENT_SCOPE must be set");

    println!("tenant_id: {}", tenant_id);
    println!("client_id: {}", client_id);
    println!("client_secret: {}", client_secret);
    println!("scope: {}", scope);

    let client = ApiClient::with_azure_sp(
        &format!("https://graph.microsoft.com/v1.0"),
        &tenant_id, &client_id, &client_secret, &scope);

    let auth_header = client.get_auth_header().await.expect("Failed to get auth header");
    println!("Auth header: {}", auth_header);
}
