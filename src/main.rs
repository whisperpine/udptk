#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;

    let args = udptk::cli::Args::parse();
    let log_level = args.get_log_level()?;
    init_tracing_subscriber(log_level);
    args.run_sub_cmd().await.unwrap_or_else(|error| {
        tracing::error!("{}", error);
    });

    Ok(())
}

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
