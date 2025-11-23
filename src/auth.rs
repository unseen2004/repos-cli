use crate::config::save_global_config;
use anyhow::{Result, Context};
use dialoguer::Password;
use octocrab::Octocrab;

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
    
    println!("Successfully authenticated as {}", user.login);

    let mut config = crate::config::load_global_config()?;
    config.github_token = Some(token);
    config.github_username = Some(user.login);
    save_global_config(&config)?;

    println!("Token saved to configuration.");
    Ok(())
}

pub fn logout() -> Result<()> {
    let mut config = crate::config::load_global_config()?;
    config.github_token = None;
    config.github_username = None;
    save_global_config(&config)?;
    println!("Logged out successfully.");
    Ok(())
}

pub fn get_token() -> Result<String> {
    let config = crate::config::load_global_config()?;
    config.github_token.context("Not logged in. Please run 'repos auth login'")
}
