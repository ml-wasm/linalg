import { FloatsMatrix } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';
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
    // const a = new FloatsMatrix([[1, 2], [3, 4]]);
    // const b = new FloatsMatrix([[5, 6], [7, 8]]);

    // const c = a.dot(b);
    // console.log(c.data);

    const a = FloatsMatrix.newWithRandom([10000, 1000]);
    const b = a.clone();

    // console.log(a.toString());
    const time = (x: any) => console.log(timeit(x)[0]);

    time(() => a.ln());
    time(() => b.lnPar());
  }
}

Comlink.expose(ScratchPad);
