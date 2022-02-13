FROM rust:latest
COPY ./ ./
RUN cargo build --release
RUN openssl req -newkey rsa:2048 -new -nodes -x509 \
    -days 3650 \
    -keyout data/cert/key.rsa \
    -out data/cert/cert.pem \
    -subj "/C=GB/ST=West-Sussex/L=Worthing/O=Scrabble AI"
EXPOSE 3030
CMD ["./target/release/server"]
