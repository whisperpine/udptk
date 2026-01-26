LOG_LEVEL := ""
#LOG_LEVEL := "-d"
#LOG_LEVEL := "-dd"

# list all available subcommands
_default:
  @just --list

# udptk help [SUBCMD]
help *SUBCMD:
  cargo run -p udptk -- {{LOG_LEVEL}} help {{SUBCMD}}

# udptk send [OPT]
send *OPT:
  cargo run -p udptk -- {{LOG_LEVEL}} send {{OPT}}

# udptk listen [OPT]
listen *OPT:
  cargo run -p udptk -- {{LOG_LEVEL}} listen {{OPT}}

# publish to crates.io (set OPT as `-n` to dry run)
publish *OPT:
  cargo publish --workspace {{OPT}}

# build the docker image for the local machine's platform
build:
  docker build -t udptk .

# build multi-platform docker images (linux/amd64,linux/arm64)
buildp:
  docker build --platform linux/amd64,linux/arm64 -t udptk .
