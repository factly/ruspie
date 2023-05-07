FROM lukemathwalker/cargo-chef:latest-rust-1.68 as chef
WORKDIR /app 

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . . 
RUN cargo build --release

FROM debian:bullseye-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/robinpie /usr/local/bin/ 
ENTRYPOINT [ "/usr/local/bin/robinpie" ]
