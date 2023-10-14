#builder
FROM rust:1.73.0 as builder

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install libpq5 -y

RUN cargo build --release

# runtime
FROM debian:12

RUN apt-get update && apt-get install libpq5 -y
RUN apt-get install libc6 -y

ENV LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu

WORKDIR /app

COPY --from=builder /app/target/release/task-manager .

RUN chmod +x task-manager

CMD ["./task-manager"]