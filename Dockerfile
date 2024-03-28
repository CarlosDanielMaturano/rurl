FROM rust:alpine as builder
RUN apk add build-base pkgconfig libressl-dev

WORKDIR /app
COPY . . 
# Only required for compiling sqlx
ENV DATABASE_URL=sqlite://database/database.db
RUN cargo run --bin setup-database
RUN cargo build --release 
RUN cp target/release/rurl .

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/rurl /app/
COPY --from=builder /app/Rocket.toml /app/
RUN mkdir logs database
ENV LOG_PATH=logs/logs.log
EXPOSE 4000
CMD ["./rurl"]
