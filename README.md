<div align="center">
  <h1>📐 ml.wasm/linalg 📏</h1>

  <p>
    <strong>Linear Algebra powered by Web Assembly</strong>
  </p>

  <sub>This project is currently in alpha and missing some crucial features</sub>

  <h3>
    <a href="https://ml-wasm.github.io/linalg">docs</a>
    <span> | </span>
    <a href="https://www.npmjs.com/package/@ml.wasm/linalg">npm</a>
  </h3>
</div>

## Quick Start

Here, to demonstrate, we'll be using [vite](https://vitejs.dev/) to scaffold a project:

``` sh
npm init vite@latest  # choose a simple vanilla js project
npm i vite-plugin-cross-origin-isolation
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
import crossOriginIsolation from 'vite-plugin-cross-origin-isolation'

export default {
  plugins: [
    crossOriginIsolation()
  ],
  server: {
    fs: {
      allow: ['..']
    }
  }
}
```
