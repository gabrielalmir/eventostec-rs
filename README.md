# eventostec

Backend da Plataforma centralizadora de eventos e meetups da comunidade tech feita para estudar conceitos fundamentais de Rust, Axum, SQLx, AWS, LocalStack, Docker & Postgres.

## Índice

- [Introdução](#introdução)
- [Tecnologias Utilizadas](#tecnologias-utilizadas)
- [Pré-requisitos](#pré-requisitos)
- [Instalação](#instalação)
- [Configuração](#configuração)

## Introdução

Este projeto foi feito para estudar conceitos fundamentais de Rust, Axum, SQLx, AWS, LocalStack, Docker & Postgres.

## Tecnologias Utilizadas

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx)
- [AWS](https://aws.amazon.com/)
- [LocalStack](https://localstack.cloud/)
- [Docker](https://www.docker.com/)
- [Postgres](https://www.postgresql.org/)
- [Docker Compose](https://docs.docker.com/compose/)


## Pré-requisitos

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [AWS CLI](https://aws.amazon.com/pt/cli/)
- [LocalStack](https://localstack.cloud/)
- [SQLx](https://github.com/launchbadge/sqlx)

## Instalação

1. Clone o repositório

```sh
git clone https://github.com/gabrielalmir/eventostec.git
```

2. Execute o comando `docker-compose up` para subir o banco de dados Postgres

```sh
docker-compose up
```

3. Execute o comando `cargo run` para subir o servidor

```sh
cargo build --release
cargo run
```

## Configuração

1. Crie um arquivo `.env` na raiz do projeto e adicione as seguintes variáveis de ambiente

```sh
DATABASE_URL=postgres://(usuário):(senha)@localhost:5432/(nome do banco)
AWS_ACCESS_KEY_ID=(chave de acesso da AWS)
AWS_SECRET_ACCESS_KEY=(chave de acesso da AWS)
AWS_BUCKET_NAME=(nome do bucket da AWS)
AWS_REGION=(us-east-1|us-west-1|us-west-2|eu-west-1|eu-central-1|ap-southeast-1|ap-southeast-2|ap-northeast-1|sa-east-1|cn-north-1|cn-northwest-1)
AWS_ENDPOINT_URI=(endereço do LocalStack)
USE_LOCALSTACK=(true|false)
```
