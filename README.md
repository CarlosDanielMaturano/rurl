# Rurl :zap:

![Static Badge](https://img.shields.io/badge/cargo-1.74.1%20-blue)

### Simple url shortner built with rust :crab: :heart: .

## Features :star:

For now, the project only has 3 core features:

- Create a new shorten url.
- View information about n existing shorten url.
- Delete a shorten url.
- Redirect to original url.

## Installation :wrench:

For using the project on your own, follow this steps:

1. Clone the project: `git clone https://github.com/CarlosDanielMaturano/rurl.git`
2. Setup the database: `cargo run --bin setup-database`
3. Run the project: `cargo run`

## Docker :whale:

The project includes a Dockerfile and a compose.yaml for containerization.

**Build the image**

- You can build the image by using: `docker build . -t rurl`
  It will a create a new image called 'rurl'

**Run the image**

- For running the image use: `docker run -p 4000:4000 rurl`
  And the api will be running under: `http://0.0.0.0:4000`

**Compose**

- You can skip this hole process just by running: `docker-compose up`

## Usage :closed_book:

See [Docs](./Docs.md).

## License :balance_scale:

The project is under [MIT](./LICENSE) license.
