# Game of Life

Conway's Game of Life implemented in Rust (compiled to WebAssembly) with a TypeScript/Vite web client.

## Prerequisites

- [Rust](https://rustup.rs/) (including `cargo`)
- [Node.js](https://nodejs.org/) (v18+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (if building locally without npm)

Note: In StackBlitz or environments with Node.js, `wasm-pack` is installed automatically via npm.

## Quick Start

From the repository root:

```sh
npm install
npm start
```

`npm install` will automatically build the Rust code to WebAssembly and install all dependencies for both the root and the web client using npm workspaces.

## Other Commands

| Command | Description |
|---|---|
| `npm run build:wasm` | Build the Rust code to WebAssembly |
| `npm run dev` | Start the Vite dev server |
| `npm run build:client` | Production build of the web client |

## Project Structure

```
├── server/               # Rust library compiled to WebAssembly
└── game-of-life-client/  # TypeScript/Vite web frontend
    └── pkg/              # Generated WASM package (git-ignored)
```
