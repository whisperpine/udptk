LOG_LEVEL := ""
#LOG_LEVEL := "-d"
#LOG_LEVEL := "-dd"

# udptk help
help *SUBCMD:
    cargo run -p udptk -- {{LOG_LEVEL}} help {{SUBCMD}}

# udptk send
send *OPT:
    cargo run -p udptk -- {{LOG_LEVEL}} send {{OPT}}

# udptk listen
listen *OPT:
    cargo run -p udptk -- {{LOG_LEVEL}} listen {{OPT}}

publish *OPT:
    cargo -Z package-workspace publish --workspace {{OPT}}
