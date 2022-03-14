import { FloatsVector } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';

export class ScratchPad {
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);
  }
  test() {
    const a = new FloatsVector([1, 2, 3, 4, 5]);

    a.powi(2);

    console.log(a.data);
  }
}

Comlink.expose(ScratchPad);
