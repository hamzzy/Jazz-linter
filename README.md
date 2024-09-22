# Rust-based Linter for JavaScript and TypeScript (WASM)

A fast, modular linter built in Rust with pluggable rules for JavaScript and TypeScript. This linter leverages the power of WebAssembly (WASM) for high-performance linting, making it accessible in Node.js environments.

## Features

- **Modular Architecture**: Each linting rule is implemented as a pluggable module, allowing for easy extensibility.
- **Performance**: Built in Rust and compiled to WebAssembly for fast execution in Node.js.
- **Customizable Rules**: Add or remove linting rules dynamically by implementing additional modules.
- **WASM for Node.js**: Compiled to WebAssembly for seamless integration with Node.js.



# Rust JavaScript Linter

A fast and efficient JavaScript linter written in Rust and compiled to WebAssembly.

## Features

- Detects unused variables
- Enforces the use of `const` where appropriate
- Checks for consistent indentation
- Compiles to WebAssembly for use in Node.js or browser environments
- Provides both programmatic and CLI interfaces

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v14 or later)

## Building

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-js-linter.git
   cd rust-js-linter
   ```

2. Build the WebAssembly module:
   ```
   wasm-pack build --target nodejs
   ```

This will create a `pkg` directory containing the compiled WebAssembly module and JavaScript bindings.

## Usage

### As a Node.js module

1. Install the local package:
   ```
   npm link ./pkg
   ```

2. Use in your JavaScript code:
   ```javascript
   const { Linter } = require('rust-js-linter');

   const linter = new Linter();
   const code = 'let x = 5; const y = 10;';
   const issues = JSON.parse(linter.lint(code));

   console.log(issues);
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.