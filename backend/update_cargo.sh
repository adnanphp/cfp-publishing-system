#!/bin/bash

# Add missing dependencies
cargo add rand --features "std_rng,distributions"
cargo add redis --features "tokio-comp,aio,connection-manager"
cargo add sqlx --features "postgres,uuid,chrono,macros,runtime-tokio-rustls,array"
cargo add async-trait
cargo add thiserror
cargo add argon2
cargo add jsonwebtoken
cargo add bcrypt

echo "Dependencies updated"
