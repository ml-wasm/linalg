import { FloatsMatrix } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';
import { Lib, Array1d, Array2d } from './utils/types'
// import { timeit } from './utils/timeit';


export interface Linalg {
  wasm: null | Lib,
  initialize: () => Promise<void>,
  createDemo: (aSize: Array1d, bSize: Array1d, cSize: Array1d) => Promise<Array<Array2d>>,
  calculate: (functionBody: String, a: Array2d, b: Array2d, c: Array2d) => any,
}

const linalg: Linalg = {
  wasm: null,
  async initialize() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);

    this.wasm = wasm;
  },
  async createDemo(aSize, bSize, cSize) {
    const random = this.wasm?.FloatsMatrix?.newWithRandomUniform;
    if (random === undefined) return new Array<Array2d>();

    const a = random(aSize, -10, 10);
    const b = random(bSize, -10, 10);
    const c = random(cSize, -10, 10);

    return [a.data, b.data, c.data];
  },
  async calculate(functionBody, aData, bData, cData) {
    const FloatsMatrix = this.wasm?.FloatsMatrix;
    if (FloatsMatrix == undefined) return null;

    const a = new FloatsMatrix(aData);
    const b = new FloatsMatrix(bData);
    const c = new FloatsMatrix(cData);

    let [result, changed] = new Function(
      'a', 'b', 'c', `return [${functionBody}, [a, b, c]]`)(a, b, c);

    if (!result) return { changed: changed.map((x: FloatsMatrix) => x.data) };

    if (typeof result === 'number') return { result: [[result]] };
    else result = result.data;

    if (!result[0]?.hasOwnProperty('length')) result = [result];

    return { result };
  },
}


Comlink.expose(linalg);
