Sure, here's the README.md file:

# Weather API

![Rust](https://img.shields.io/badge/Rust-1.55.0-orange.svg?style=flat-square)
![Actix-Web](https://img.shields.io/badge/Actix--Web-4.0.0-blue.svg?style=flat-square)
![Docker](https://img.shields.io/badge/Docker-20.10.8-blue.svg?style=flat-square)

This is a simple weather API built using Rust and Actix-Web.

## Installation

To run this application, you will need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Running the Application

To run the application, navigate to the root directory of the project and run the following command:

```bash
cargo run
```

This will start the application on `http://localhost:8080/weather`.

## Running the Application with Docker

To run the application using Docker, you will need to have Docker installed on your system. You can install Docker by following the instructions on the [official Docker website](https://docs.docker.com/get-docker/).

Once you have Docker installed, navigate to the root directory of the project and run the following command:

```bash
docker build -t weather-api .
```

This will build a Docker image of the application.

To run the Docker image, run the following command:

```bash
docker run -p 8080:8080 weather-api
```

This will start the application on `http://localhost:8080/weather`.

## Files

- `main.rs`: This is the main file of the application. It defines the routes and handlers for the API.
- `Dockerfile`: This file is used to build a Docker image of the application. It uses a multi-stage build process to optimize the size of the final image.