---
slug: /
title: Getting Started
---

The goal of this project is to provide a **fast, easy-to-use linear algebra
Javascript library** powered by WebAssembly, written in Rust.

This project is a part of a larger project called [wasml](https://www.github.com/wasml),
which aims to provide a complete machine learning ecosystem in JavaScript.

## Installing the package

_Coming Soon_

## How to use

```js
import init, {
  initThreadPool,
  Integers1d,
  Floats1d,
  Strings1d,
  Integers2d,
  Floats2d,
  Strings2d,
} from '@wasml/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  await initThreadPool(navigator.hardwareConcurrency);

  // All your code goes here...
})();
```

:::warning Proper error handling not implemented yet

If you are familiar with Rust, currently all the functions just "panic". This
will be fixed in the future.

:::

For more information on how to work with one dimensional arrays or vectors go
[here](One%20Dimensional/index). For two dimensional arrays or matrices go
[here](Two%20Dimensional/index).
