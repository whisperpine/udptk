#!/bin/sh

docker build -t udptk . \
    --platform linux/amd64,linux/arm64 \
    --pull
