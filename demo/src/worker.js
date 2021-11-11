import * as Comlink from 'comlink';

const linalg = {
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
  async calculate(functionBody) {
    const a = new this.wasm.FloatsMatrix([[1, 2, 3], [4, 5, 6]]);
    const b = new this.wasm.FloatsMatrix([[-1, -2, -3], [-4, -5, -6]]);
    const c = new this.wasm.FloatsMatrix([[1, 2], [3, 4], [5, 6]]);

    let result = new Function('a', 'b', 'c', `return ${functionBody}`)(a, b, c);
    if (typeof result === 'number') result = [[result]];
    else result = result.data;

    if (!result[0]?.hasOwnProperty('length')) result = [result];

    return result;
  },
}


Comlink.expose(linalg);
