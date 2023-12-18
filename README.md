# Game of Life (Web-Assembly)

This project is a Rust implementation of Conway's Game of Life, compiled to WebAssembly.

## Prerequisites

- Rust and Cargo: You can download these from the [official site](https://www.rust-lang.org/tools/install).
- wasm-pack: This is a tool for assembling and packaging Rust crates that target WebAssembly. Install it with `cargo install wasm-pack`.

## Building the Project

1. Navigate to the project directory.
2. Run `wasm-pack build --target web` to compile the Rust code into WebAssembly.

## Hosting the App

After building the project, the `pkg/` directory will contain the compiled WebAssembly code. You can use a simple HTTP server to serve the static files.

1. Install a static server like `basic-http-server` with `cargo install basic-http-server`.
2. Run `basic-http-server` to start the server (This will serve the `index.html` file in the project root).
3. Open your web browser and navigate to `localhost:4000` (or the port number displayed in your terminal).

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.