//! Command line interface.

use clap::{Parser, Subcommand};

/// UDP Toolkit
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    subcommands: Option<SubCommands>,
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
    #[arg(short, long)]
    port: u16,
    #[arg(short, long)]
    domain: Option<String>,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        if let Some(sub_cmd) = self.subcommands {
            match sub_cmd {
                SubCommands::Send(a) => udptk::send(a.domain, a.port, a.content).await?,
                SubCommands::Listen { port } => udptk::listen(port).await?,
            }
        }
        Ok(())
    }
}
