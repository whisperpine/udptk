//! Command line interface.

use clap::{Parser, Subcommand};

/// UDP Toolkit
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    subcommands: SubCommands,
    // subcommands: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Send
    Send(SendArgs),
    /// Listen
    Listen {
        /// port
        #[arg(short, long)]
        port: u16,
    },
}

#[derive(clap::Args)]
struct SendArgs {
    /// Anything you want to send
    content: String,
    /// Supported formats: "domain:port", "ip:port"
    #[arg(short, long)]
    #[arg(default_value_t = format!("localhost:6777"))]
    target: String,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        match self.subcommands {
            SubCommands::Send(a) => udptk::send(a.target, a.content).await?,
            SubCommands::Listen { port } => udptk::listen(port).await?,
        }
        Ok(())
    }
}
