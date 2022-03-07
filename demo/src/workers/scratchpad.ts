import { FloatsVector } from '@ml.wasm/linalg';
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
  parallelComparison() {
    for (let size = 1e1; size < 1e8; size *= 10) {
      const normal = [], parallel = [];
      // const consume = [], parallelConsume = [];
      for (let i = 0; i < 1e0; i++) {
        let na = FloatsVector.newWithRandom(size);
        let pa = na.clone();
        // let ca = na.clone();
        // let pca = na.clone();

        const [n] = timeit(() => na.ln());
        na.free();
        const [p] = timeit(() => pa.lnPar());
        pa.free();
        // const [c, cra] = timeit(() => ca.ln_());
        // cra.free();
        // const [pc, pcra] = timeit(() => pca.lnPar_());
        // pcra.free();

        normal.push(n);
        parallel.push(p);
        // consume.push(c);
        // parallelConsume.push(pc);
      }
      console.log(`size: ${size} ln: ${average(normal)} lnPar: ${average(parallel)}` /* average(consume), average(parallelConsume) */);
    }
  }
  test() {

  }
}

Comlink.expose(ScratchPad);
