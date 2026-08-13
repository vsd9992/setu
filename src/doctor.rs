use crate::{config::Config, error::SetuError, joplin::JoplinClient};

const EXPECTED_PING: &str = "JoplinClipperServer";

pub async fn run_doctor(config: &Config) -> Result<(), SetuError> {
    println!("✓ Configuration is valid and restricted to loopback");

    let client = JoplinClient::new(config)?;
    let body = client.ping().await?;

    if body.trim() != EXPECTED_PING {
        return Err(SetuError::UnexpectedJoplinResponse);
    }

    println!("✓ Joplin clipper service is reachable");
    if client.has_token() {
        client.check_authentication_without_content().await?;
        println!("✓ Joplin authentication succeeded without retrieving note content");

        if config.allowed_notebooks.is_empty() {
            println!("ℹ No notebook allowlist is configured");
        } else {
            let notebooks = client
                .resolve_allowed_notebooks(&config.allowed_notebooks)
                .await?;
            println!(
                "✓ Resolved {} explicitly allowed notebook(s)",
                notebooks.len()
            );
        }
    } else {
        println!("ℹ JOPLIN_TOKEN is not configured; authentication was skipped");
        if !config.allowed_notebooks.is_empty() {
            return Err(SetuError::NotebookAllowlistRequiresToken);
        }
    }

    Ok(())
}
