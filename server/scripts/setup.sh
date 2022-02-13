echo "create directories"
mkdir -p data/db
mkdir -p data/cert

echo "run sqlx migrate"
sqlx migrate run

echo "generate certificate"
openssl req -newkey rsa:2048 -new -nodes -x509 \
    -days 3650 \
    -keyout data/cert/key.rsa \
    -out data/cert/cert.pem \
    -subj "/C=GB/ST=West-Sussex/L=Worthing/O=Scrabble AI"
