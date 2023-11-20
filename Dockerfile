# Dockerfile simples,
# Utilizado para desenvolvimento.
# Tamanho da imagem será bem grande.
# Porém, quando fazer mudanças, basta reiniciar o container e não será necessário esperar o sistema compilar todo o código do começo novamente.

FROM rust:latest

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src/
COPY ./static/ ./static/
COPY ./lib/ ./lib/

RUN cargo build --release

EXPOSE 8080

CMD [ "cargo", "run", "--release" ]