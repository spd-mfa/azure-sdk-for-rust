use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_security_keyvault::KeyClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let backup_blob = env::var("BACKUP_BLOB").expect("Missing BACKUP_BLOB environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );
    let mut client = KeyClient::new(&keyvault_url, &creds)?;

    client.restore_secret(&backup_blob).await?;

    Ok(())
}
