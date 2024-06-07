//! Command line interface.

use clap::{Parser, Subcommand};

/// The main command line interface.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Increase logging verbosity.
    ///
    /// This flag can be set multiple times to increase verbosity.
    #[arg(short, action = clap::ArgAction::Count,
        help = "Increase logging verbosity. \n\
        This flag can be set multiple times to increase verbosity.")]
    debug: u8,
    /// The subcommand to run.
    #[command(subcommand)]
    subcommands: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    /// Send UDP packets.
    Send(SendArgs),
    /// Listen to a UDP port.
    Listen {
        /// Port to bind with.
        #[arg(short, long)]
        port: u16,
    },
}

#[derive(clap::Args, Debug)]
struct SendArgs {
    /// The content of the UDP packet to send.
    content: String,
    /// The address to send UDP packets to.
    ///
    /// Example: "localhost:6777"
    #[arg(short, long)]
    #[arg(
        default_value_t = format!("localhost:6777"),
        help = "The address to send UDP packets to.\n\
                Format: [domain:port] or [ip:port]."
    )]
    target: String,
}

impl Args {
    /// Get the log level based on the number of times the debug flag is used.
    ///
    /// The log level can be one of:
    /// * `info`: if `-d` is not used.
    /// * `debug`: if `-d` is used once.
    /// * `trace`: if `-d` is used twice (e.g. `-dd`).
    ///
    /// If the debug flag is used more than twice, it will return an error.
    pub fn get_log_level(&self) -> anyhow::Result<&'static str> {
        match self.debug {
            0 => Ok("info"),
            1 => Ok("debug"),
            2 => Ok("trace"),
            _ => anyhow::bail!(r#"debug flat "-d" can only be set up to 2 times (e.g. "-dd")"#),
        }
    }

    /// Run the subcommand specified in the command line arguments.
    pub async fn run_sub_cmd(&self) -> anyhow::Result<()> {
        match &self.subcommands {
            SubCommands::Send(a) => crate::send(&a.target, &a.content).await?,
            SubCommands::Listen { port } => crate::listen(*port).await?,
        }
        Ok(())
    }
}
