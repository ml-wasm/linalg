import { BytesVector } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';
import { average } from '../utils/stats';
import { timeit } from '../utils/timeit';

export class ScratchPad {
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);
  }
  test() {
    const a = BytesVector.newWithRandom(1e2);
    console.log(a.data);
  }
}

Comlink.expose(ScratchPad);
