import * as Comlink from 'comlink';

export class ImageManipulation {
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`initialized (threads: ${threads})`);
  }
}

Comlink.expose(ImageManipulation);
