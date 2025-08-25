# Microservices-demo-rs

This is a reimplementation of Google's [microservices demo application](https://github.com/GoogleCloudPlatform/microservices-demo) with all services written in Rust.

The services communicate via [`tarpc`](https://github.com/google/tarpc), an RPC framework written in Rust that defines the services via the Rust trait system.

The fronting webserver is written in [Rocket](https://rocket.rs/).

To make the application slightly more compelling and realistic in terms of sensitive information usage, we augment the baseline application to store orders and payment information.
Each relevant service only stores the relevant information required for their local processing:
    - The payment service now stores a transaction ID with the amounts. If the user wants to, they can store their payment information for future use.
    - The shipping service stores the package tracking number and the associated address.
    - The checkout service stores the order, comprised of its cart contents, and the associated transactionId and trackingId.

Additionally, we allow for ads to be targeted based on the user's address for targeted content.

# Advisory

This demo application is used as an application-level reimplementation of the services.
We do not handle containerization nor orchestration, contrary to the Google's repo.

We knowingly do not use the most popular tools for both gRPC and web frameworks.
