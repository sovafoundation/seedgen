# --- Cargo Chef planner (Debian/glibc) ---
FROM lukemathwalker/cargo-chef:latest-rust-1.88 AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Cargo Chef cook/deps (Debian/glibc) ---
FROM lukemathwalker/cargo-chef:latest-rust-1.88 AS cooker
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json

# --- Build app (Debian/glibc) ---
FROM rust:1.88-slim AS build
WORKDIR /app
COPY . .
COPY --from=cooker /app/target target
RUN cargo build --release --locked

# --- Distroless runtime (Debian/glibc) ---
FROM gcr.io/distroless/cc-debian12:nonroot AS run
WORKDIR /app
COPY --from=build /app/target/release/seedgen /usr/local/bin/seedgen
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/seedgen"]
