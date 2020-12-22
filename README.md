# Marrow

[简体中文](README.zh.md)

`Marrow` is a safety runtime for unversal device. Now `Marrow` use `Webassembly` as runtime.

## Usage

runtime is a TypeScript module to provide wasm runtime and schedule wasm module to run.

To use it:

1. Make sure your machine has the latest version of nodeJS and npm installed.

2. Add the compiled wasm module information to the modules array in the `runtime/index.ts` file.

3. Run the following command in the `runtime` directory:

```sh
npm run dev
```

## Design

`Marrow` can load multiple `Webassembly` modules, these modules call each other through `CABI`. `Marrow` provides standard libraries in multiple languages, that can access resource for physical node.

In the future, `Marrow` will support running modules in distributed.

![design](docs/assets/design.png)

### ABI Call Manager

### ABI Filter

## Versions

### Single thread runtime for javascript

> Current version.

This version can apply for browser and server.

### Single thread runtime for rust

### Distributed runtime
