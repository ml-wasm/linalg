import * as Comlink from 'comlink';
import { timeit } from './utils';

interface Linalg {
  wasm: null | typeof import("/home/puipuituipui/Projects/ml.wasm/linalg/pkg/linalg");
  initialize: () => Promise<any>,
  calculate: (functionBody: String) => any,
  test: () => any,
}


const linalg: Linalg = {
  wasm: null,
  async initialize() {
    console.log('inits started');
    const wasm = await import('@ml.wasm/linalg');

    await wasm.default();
    await wasm.initThreadPool(navigator.hardwareConcurrency);

    console.log(`inits completed (threads: ${navigator.hardwareConcurrency})`);

    this.wasm = wasm;

    const a = new this.wasm.FloatsMatrix([[1, 2, 3], [4, 5, 6]]);
    const b = new this.wasm.FloatsMatrix([[-1, -2, -3], [-4, -5, -6]]);
    const c = new this.wasm.FloatsMatrix([[1, 2], [3, 4], [5, 6]]);

    return [a.data, b.data, c.data];
  },
  async calculate(functionBody: String) {
    const a = new this.wasm.FloatsMatrix([[1, 2, 3], [4, 5, 6]]);
    const b = new this.wasm.FloatsMatrix([[-1, -2, -3], [-4, -5, -6]]);
    const c = new this.wasm.FloatsMatrix([[1, 2], [3, 4], [5, 6]]);

    let result = new Function('a', 'b', 'c', `return ${functionBody}`)(a, b, c);
    if (typeof result === 'number') result = [[result]];
    else result = result.data;

    if (!result[0]?.hasOwnProperty('length')) result = [result];

    return result;
  },
  async test() {
    const { FloatsVector } = linalg.wasm;
    const a = FloatsVector.newWithRandom(5e6);
    const b = a.clone();

    const [normal] = timeit(() =>
      a.MulAdd(9.87654321, 1.23456789)
    );
    const [parallel] = timeit(() =>
      b.MulAdd_(9.87654321, 1.23456789)
    );

    console.log(`normal ${normal}`);
    console.log(`parallel ${parallel}`);
  }
}


Comlink.expose(linalg);
