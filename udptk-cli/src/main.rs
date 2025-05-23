//! UDP toolkit cli.

// rustc
#![cfg_attr(debug_assertions, allow(unused))]
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![cfg_attr(not(debug_assertions), deny(clippy::unwrap_used))]
#![cfg_attr(not(debug_assertions), deny(warnings))]
// clippy
#![cfg_attr(not(debug_assertions), deny(clippy::todo))]
#![cfg_attr(
    not(any(test, debug_assertions)),
    deny(clippy::print_stdout, clippy::dbg_macro)
)]

mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;

    // Parse command line arguments
    let args = crate::cli::Args::parse();

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
                .unwrap_or_else(|_| format!("udptk={log_level}").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
