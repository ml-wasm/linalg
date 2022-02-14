import * as Comlink from 'comlink';
import { Linalg } from '../utils/types';

export class KnightDemo {
  wasm!: Linalg;
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);

    this.wasm = wasm;
  }
  test() {
    console.log((new this.wasm.FloatsVector([1, 2, 3])).toString());
  }
}

Comlink.expose(KnightDemo);
