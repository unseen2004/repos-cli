use crate::config::{save_global_config, load_global_config};
use anyhow::{Result, Context};
use dialoguer::Password;
use octocrab::Octocrab;
use keyring::Entry;
use tracing::{info, error};

const SERVICE_NAME: &str = "repos-cli";
const USER_KEY: &str = "github_token";

pub async fn login() -> Result<()> {
    println!("Please enter your GitHub Personal Access Token (PAT).");
    println!("Permissions required: repo, read:user");
    
    let token = Password::new()
        .with_prompt("GitHub Token")
        .interact()?;

    // Validate token
    let octocrab = Octocrab::builder()
        .personal_token(token.clone())
        .build()?;

    let user = octocrab.current().user().await.context("Failed to authenticate with GitHub")?;
    
    info!("Successfully authenticated as {}", user.login);
    println!("Successfully authenticated as {}", user.login);

    // Save token to keyring
    let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
    entry.set_password(&token)?;

    let mut config = load_global_config()?;
    config.github_username = Some(user.login);
    save_global_config(&config)?;

    println!("Token saved securely to system keyring.");
    Ok(())
}

pub fn logout() -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
    match entry.delete_credential() {
        Ok(_) => println!("Token removed from keyring."),
        Err(e) => error!("Failed to remove token from keyring: {}", e),
    }

    let mut config = load_global_config()?;
    config.github_username = None;
    save_global_config(&config)?;
    println!("Logged out successfully.");
    Ok(())
}

pub fn get_token() -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
    entry.get_password().context("Not logged in. Please run 'repos auth login'")
}
