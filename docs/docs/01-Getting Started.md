---
slug: /
title: Getting Started
---

The goal of this project is to provide a **fast, easy-to-use linear algebra
Javascript library** powered by WebAssembly, written in Rust.

This project is a part of a larger project called [ml.wasm](https://www.github.com/ml.wasm),
which aims to provide a complete machine learning ecosystem in JavaScript.

## Quick Start

Here, to demonstrate, we'll be using [vite](https://vitejs.dev/) to scaffold a project:

``` sh
npm init vite@latest  # choose a simple vanilla js project
npm i @ml.wasm/linalg
```

And add this to `main.js`:

```js title="main.js"
import init, {
  initThreadPool,
  IntegersVector,
  FloatsVector,
  StringsVector,
  IntegersMatrix,
  FloatsMatrix,
  StringsMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  await initThreadPool(navigator.hardwareConcurrency);

  // All your code goes here...
})();
```

Note that you'll have to create a `vite.config.js`:

``` js title="vite.config.js"
export default {
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ['..']
    }
  }
}
```

:::warning Proper error handling not implemented yet

If you are familiar with Rust, currently all the functions just "panic". This
will be fixed in the future.

:::

For more information on how to work with one dimensional arrays or vectors go
[here](Vectors/index). For two dimensional arrays or matrices go
[here](Matrices/index).
