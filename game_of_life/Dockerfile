FROM rust:1.85 AS builder

RUN mkdir -p /app
WORKDIR /app
COPY . .
RUN cargo build --release -F headless


FROM python:3 AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/game_of_life /app/
COPY ./maps ./maps
COPY ./run.py ./run.py
CMD python3 run.py
