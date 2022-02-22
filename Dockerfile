FROM rust:latest
COPY ./server ./server
COPY ./api ./api
COPY ./scrabble ./scrabble
WORKDIR "/server"
ENV RUST_LOG=trace
RUN cargo build --release
RUN mkdir -p data/db
RUN mkdir -p data/cert
RUN openssl req -newkey rsa:2048 -new -nodes -x509 \
    -days 3650 \
    -keyout data/cert/key.rsa \
    -out data/cert/cert.pem \
    -subj "/C=GB/ST=West-Sussex/L=Worthing/O=Scrabble AI"
EXPOSE 3030
CMD ["./target/release/server"]