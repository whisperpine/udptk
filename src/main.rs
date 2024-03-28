mod cli;

fn main() {
    use clap::Parser;

    init_tracing_subscriber();
    cli::Args::parse().run().unwrap_or_else(|error| {
        tracing::error!("{}", error);
    });
}

fn init_tracing_subscriber() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", udptk::CRATE_NAME).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
