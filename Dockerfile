FROM rust:1.64
LABEL org.opencontainers.image.authors="maxime.lestage@ynov.com"
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo build --release
CMD ./target/release/tp_wik_dps_tp01
EXPOSE 9000
