import { FloatsMatrix } from '@ml.wasm/linalg';
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
    let a: FloatsMatrix = FloatsMatrix.newWithZeroes(3, 3 * 4);
    for (let i = 0; i < 3; ++i) {
      for (let j = 0; j < 3 * 4; j += 4) {
        a.set([i, j], 0);
        a.set([i, j + 1], 1);
        a.set([i, j + 2], 2);
        a.set([i, j + 3], 3);
      }
    }
    let b = a.toVector().toTypedArray();
    console.log(b.toString());
  }
}

Comlink.expose(KnightDemo);
