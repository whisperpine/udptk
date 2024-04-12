#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;

    // Parse command line arguments
    let args = udptk::cli::Args::parse();

    // Initialize tracing subscriber with log level from args
    let log_level = args.get_log_level()?;
    init_tracing_subscriber(log_level);

    // Run subcommand with given args
    tracing::trace!("Running sub-command with args: {:?}", args);
    args.run_sub_cmd().await.unwrap_or_else(|error| {
        tracing::error!("{}", error);
    });

    Ok(())
}

/// Initialize tracing subscriber with default environment variable fallback
/// for log level.
///
/// The log level can be specified via the `RUST_LOG` environment variable.\
/// If the environment variable is not set, it falls back to the
/// given `log_level` parameter.
///
/// This function is called in the `main` function of the binary to
/// initialize the logging infrastructure before the application logic
/// starts.
fn init_tracing_subscriber(log_level: &str) {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}={}", udptk::CRATE_NAME, log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
