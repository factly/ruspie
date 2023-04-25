FROM lukemathwalker/cargo-chef:latest-rust-1.63 as chef
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
COPY --from=builder /app/target/release/robinpie /usr/local/bin/ 
ENTRYPOINT [ "/usr/local/bin/robinpie" ]
