use common::{config::server::ServerProtocol, manager::boot::BootManager};
use imap::core::{ImapSessionManager, IMAP};
use pop3::Pop3SessionManager;
use jmap::{api::JmapSessionManager, services::IPC_CHANNEL_BUFFER, JMAP};
use managesieve::core::ManageSieveSessionManager;
use smtp::core::{SmtpSessionManager, SMTP};
use tokio::sync::mpsc;
use utils::wait_for_shutdown;

use std::{
  time::Duration,
  env,
};

pub async fn start_mail_server() -> std::io::Result<()> {
  let init = BootManager::init().await;

    // Parse core
//    let key = "CONFIG_PATH";
//    env::set_var(key, "./config.toml");
    std::env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.toml".into());
    let mut config = init.config;
    let core = init.core;

    // Init servers
    let (delivery_tx, delivery_rx) = mpsc::channel(IPC_CHANNEL_BUFFER);
    let smtp = SMTP::init(&mut config, core.clone(), delivery_tx).await;
    let jmap = JMAP::init(&mut config, delivery_rx, core.clone(), smtp.inner.clone()).await;
    let imap = IMAP::init(&mut config, jmap.clone()).await;
   
    tracing::info!("Starting mail server");
    // Log configuration errors
    config.log_errors(init.guards.is_none());
    config.log_warnings(init.guards.is_none());

    // Spawn servers
    let (shutdown_tx, shutdown_rx) = init.servers.spawn(|server, acceptor, shutdown_rx| {
        match &server.protocol {
            ServerProtocol::Smtp | ServerProtocol::Lmtp => server.spawn(
                SmtpSessionManager::new(smtp.clone()),
                core.clone(),
                acceptor,
                shutdown_rx,
            ),
            ServerProtocol::Http => server.spawn(
                JmapSessionManager::new(jmap.clone()),
                core.clone(),
                acceptor,
                shutdown_rx,
            ),
            ServerProtocol::Imap => server.spawn(
                ImapSessionManager::new(imap.clone()),
                core.clone(),
                acceptor,
                shutdown_rx,
            ),
	    ServerProtocol::Pop3 => server.spawn(
                Pop3SessionManager::new(imap.clone()),
                core.clone(),
                acceptor,
                shutdown_rx,
            ),
            ServerProtocol::ManageSieve => server.spawn(
                ManageSieveSessionManager::new(imap.clone()),
                core.clone(),
                acceptor,
                shutdown_rx,
            ),
        };
    });

    // Wait for shutdown signal
    wait_for_shutdown(&format!(
        "Shutting down Stalwart Mail Server v{}...",
        env!("CARGO_PKG_VERSION")
    ))
    .await;

    // Stop services
    let _ = shutdown_tx.send(true);

    // Wait for services to finish
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
