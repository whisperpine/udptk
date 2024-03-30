//! Command line interface.

use clap::{Parser, Subcommand};

/// UDP Toolkit
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Turn debug information on
    #[arg(short, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    subcommands: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Send UDP packets
    Send(SendArgs),
    /// Listen to a UDP port
    Listen {
        /// Port to bind with
        #[arg(short, long)]
        port: u16,
    },
}

#[derive(clap::Args)]
struct SendArgs {
    /// Anything you want to send
    content: String,
    /// "domain:port" or "ip:port"
    #[arg(short, long)]
    #[arg(default_value_t = format!("localhost:6777"))]
    target: String,
}

impl Args {
    /// Get log level from `-d` flag repeat times.
    pub fn get_log_level(&self) -> anyhow::Result<&'static str> {
        match self.debug {
            0 => Ok("info"),
            1 => Ok("debug"),
            2 => Ok("trace"),
            _ => anyhow::bail!(r#"debug flat "-d" can only be set up to 2 times (e.g. "-dd")"#),
        }
    }

    /// Run sub commands.
    pub async fn run_sub_cmd(&self) -> anyhow::Result<()> {
        match &self.subcommands {
            SubCommands::Send(a) => udptk::send(&a.target, &a.content).await?,
            SubCommands::Listen { port } => udptk::listen(*port).await?,
        }
        Ok(())
    }
}
