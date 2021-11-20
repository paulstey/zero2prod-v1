# Planner stage
FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 AS chef
WORKDIR /app

FROM chef AS planner 
COPY . .

# Compute a lock-like file for our project 
RUN cargo chef prepare --recipe-path recipe.json






# Builder stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json 

# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

# Up to this point, if our dependency tree stays the same,
# all layers should be cached 
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod







# Runtime stage 
FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment
# to our runtime environment.
COPY --from=builder /app/target/release/zero2prod zero2prod

# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT ["./target/release/zero2prod"]