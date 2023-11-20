# simple_encryption

Exemplo simples de Criptografia RSA e AES

## Como executar

Apenas Docker é necessário para iniciar o servidor.

```bash
# Monta o container
docker compose build
# Executa o container
docker compose up
```

Basta acessar o website pelo browser no endereço `http://localhost:8080`.

Caso tenha Rust instalado:

```bash
# Compila o código
cargo build --release
# Executa testes unitários na crate principal
cargo test --release
# Executa testes unitários na crate aes_simple
cargo test --release -p aes_simple
# Executa testes unitários na crate rsa_simple
cargo test --release -p rsa_simple
```

É possivel gerar um website com toda a documentação deste código.

```bash
# Gera e abre o website
cargo doc --open
# Gera o website, pode ser acessado no caminho /target/doc/simple_encryption/index.html
cargo doc
```
