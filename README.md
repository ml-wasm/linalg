<div align="center">
  <h1>📐 ml.wasm/linalg 📏</h1>

  <p>
    <strong>Linear Algebra powered by Web Assembly</strong>
  </p>

  <h3>
    <a href="https://ml-wasm.github.io/linalg">DOCS</a>
    <span> | </span>
    <a href="https://www.npmjs.com/package/@ml.wasm/linalg">NPM</a>
  </h3>
  
  <sub>This project is currently in alpha and missing some crucial features</sub>
</div>

# Quick Start

Add to your project:

``` sh
npm i @ml.wasm/linalg
```

Use in Javascript:

``` javascript
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
