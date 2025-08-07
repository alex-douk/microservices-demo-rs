# Microservices-demo-rs

This is a reimplementation of Google's [microservices demo application](https://github.com/GoogleCloudPlatform/microservices-demo) with all services written in Rust.

The services communicate via [`tarpc`](https://github.com/google/tarpc), an RPC framework written in Rust that defines the services via the Rust trait system.

The fronting webserver is written in [Rocket](https://rocket.rs/).


# Advisory

This demo application is used as an application-level reimplementation of the services.
We do not handle containerization nor orchestration, contrary to the Google's repo.

We knowingly do not use the most popular tools for both gRPC and web frameworks.
