use quantaura::*;

use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::services::trading_runtime::service::TradingRuntimeService;

#[tokio::main]
async fn main() {
    let state = state::AppState::new().await;
    let config = state.config.clone();
    init_tracing(&config.logging.level);

    let trading_runtime = state.services.trading_runtime_service.as_ref().clone();

    match trading_runtime.recover_running_traders_from_db().await {
        Ok(recovered) => {
            if recovered.is_empty() {
                info!("startup recovery found no running traders to resume");
            } else {
                info!("startup recovery resumed {} trader(s)", recovered.len());
            }
        }
        Err(err) => {
            error!("startup recovery failed: {err}");
        }
    }

    let app = routes::build_app(state, config.server.request_timeout_secs);

    let addr = config.server_addr();
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("failed to bind {addr}: {err}");
            std::process::exit(1);
        }
    };

    info!(
        "QUANTAURA API listening on http://{} (env={}, app={})",
        listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| addr.to_string()),
        config.app.environment,
        config.app.name
    );

    let server =
        axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(trading_runtime));

    if let Err(err) = server.await {
        error!("server error: {err}");
        std::process::exit(1);
    }
}

fn init_tracing(log_level: &str) {
    let filter = format!("quantaura={log_level},axum={log_level},tower_http={log_level}");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn shutdown_signal(trading_runtime: TradingRuntimeService) {
    let ctrl_c = async {
        if let Err(err) = tokio::signal::ctrl_c().await {
            error!("failed to install Ctrl+C handler: {err}");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut sigterm) => {
                sigterm.recv().await;
            }
            Err(err) => {
                error!("failed to install SIGTERM handler: {err}");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("shutdown signal received, stopping runtime trading engines");

    if let Err(err) = trading_runtime.shutdown_all().await {
        error!("failed to shutdown runtime trading engines cleanly: {err}");
    }
}
