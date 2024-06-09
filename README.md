# RUST Simple Web-Service With Axum-SeaORM-PostGres-MVC

## Overview
This project is a web service application implemented in Rust, using Axum framework for efficient and scalable routing. The database operations are handled by SeaORM. The project is designed using the MVC architecture to separate concerns and improve code maintainability.

## Features
- **High Performance**: Built with Rust for optimal speed and efficiency.
- **Modular Design** : Use MVC model to build a complete simple web-application 
- **FrameWork** : Axum, SeaORM 
- **Extensible**: Deploying simple modules for beginners to understand and learn about Rust

## Installation
### First you need to download the necessary tools  


1. **Clone the repository:**

    ```bash
    git clone https://github.com/PhucLam202/rust-axum-seaorm-mvc.git
    cd rust-axum-seaorm-mvc
    ```

2. **Install Rust and Cargo, SeaORM, Axum:**

    Ensure you have Rust and Cargo installed. You can install them via 
      - [Rust](https://rustup.rs/)       
      - [SeaORM](https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime/)
      - [Axum](https://crates.io/crates/axum)  

3. **Build the project:**

    ```
    cargo build
    ```

4. **Run the project:**

    ```bash
    cargo run
    ```

## Project Tree Source 

```
src
├── controllers
│   ├── mod.rs
│   └── users_controller.rs
├── main.rs
├── middleware
│   ├── api_error.rs
│   └── mod.rs
├── models
│   ├── mod.rs
│   └── users_model.rs
├── routers
│   ├── mod.rs
│   └── user_router.rs
└── server
    ├── mod.rs
    └── postgres_server.rs
```
- **`src/controllers`**: Contains the controllers for handling HTTP `requests` and `responses`.
- **`src/models`**: Contains the models representing the data structures and `database entities`.
- **`src/middleware`** :  Middleware contains manage error files or security-related files.
- **`src/routers`** :   Contains routers endpoint in the project.
- **`src/server`** : Contains managed connection to `database`.  
 