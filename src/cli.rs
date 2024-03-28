//! Command line interface.

use clap::{Parser, Subcommand};

/// UDP Toolkit
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
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
pub struct SendArgs {
    /// Anything you want to send
    pub content: String,
}

impl Args {
    pub fn run(self) -> anyhow::Result<()> {
        if let Some(sub_cmd) = self.subcommands {
            match sub_cmd {
                SubCommands::Send(send_args) => udptk::send(send_args.content)?,
                SubCommands::Listen { port } => udptk::listen(port)?,
            }
        }
        Ok(())
    }
}
