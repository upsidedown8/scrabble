# install openssl
#   choco install openssl
#   pacman -S openssl

# generate certificate
openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.rsa -out cert.pem
