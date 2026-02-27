# Game of Life

Conway's Game of Life implemented in Rust (compiled to WebAssembly) with a TypeScript/Vite web client.

## Prerequisites

- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v18+)

## Quick Start

From the repository root:

```sh
npm start
```

This single command will:

1. Build the Rust code to WebAssembly (via `wasm-pack`)
2. Install client dependencies
3. Start the Vite dev server and open the app in your browser

## Other Commands

| Command | Description |
|---|---|
| `npm run build:wasm` | Build only the WASM package |
| `npm run install:client` | Install client npm dependencies |
| `npm run build:client` | Production build of the client |
| `npm run dev` | Start the Vite dev server (assumes WASM is already built) |
| `npm run setup` | Build WASM + install client deps (without starting dev server) |

## Project Structure

```
├── server/          # Rust library compiled to WebAssembly
└── game-of-life-client/  # TypeScript/Vite web frontend
    └── pkg/         # Generated WASM package (git-ignored)
```
