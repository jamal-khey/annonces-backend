# rust backend with rocket and mangodb
[![CI](https://github.com/psychodrom/annonces-backend/actions/workflows/ci.yaml/badge.svg)](https://github.com/psychodrom/annonces-backend/actions/workflows/ci.yaml)

Fully working CRUD REST API using 
- Rust (stable)
- Rocket.rs
- mongodb
- okapi


## 🚀 Features
- Establish MongoDB connection using rocket Adhoc fairing.
- Custom error handlings with rocket Responder and okapi OpenApiGenerator.
- CORS fairing and Counter fairing to demonstrate how fairing works.
- Request guard using ApiKey.
- Implement Open API documentation using okapi.
- Test codes to test API endpoints.


## 🔧 Building and Testing

### See Makefile for all commands

### developpement mode
> make dev

### debug mode
> cargo run

### release mode
> cargo build --release && cargo run --release


### unit testing
> cargo test

<br/>

ℹ️ _You should create your own `.env` file including `MONGO_URI`, `MONGO_DB_NAME`, and `API_KEY` to run it._

## deploy

to the ploy the projet on production
> setup.sh
on the remote server to install the dependencies (rust compiler ..)
> deploy-rust.sh
to zip the code sources , copy it to the server , compile and run 