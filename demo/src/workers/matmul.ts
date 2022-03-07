import { FloatsMatrix } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';
import { idx } from '../utils/idx';
import { timeit } from '../utils/timeit';
import { Array2d } from '../utils/types';

export class MatMul {
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);
  }
  arrays(a: number[], b: number[], size: number): number {
    return timeit(() => {
      let c = Array(size * size)
      for (let i = 0; i < size; ++i) {
        for (let j = 0; j < size; ++j) {
          let sum = 0;
          for (let k = 0; k < size; ++k) {
            sum += a[idx(size, i, k)] * b[idx(size, k, j)];
          }
          c[idx(size, i, j)] = sum;
        }
      }
    })[0];
  }
  typedArrays(aa: number[], ba: number[], size: number): number {
    let a = Float64Array.from(aa), b = Float64Array.from(ba);

    return timeit(() => {
      const c = new Float64Array(size * size);
      for (let i = 0; i < size; ++i) {
        for (let j = 0; j < size; ++j) {
          let sum = 0;
          for (let k = 0; k < size; ++k) {
            sum += a[idx(size, i, k)] * b[idx(size, k, j)];
          }
          c[idx(size, i, j)] = sum;
        }
      }
    })[0];
  }
  linalg(aa: number[], ba: number[], size: number): number {
    const convert = (x: number[]) => {
      const res: Array2d = [];
      for (let i = 0; i < size; ++i) {
        res.push([]);
        for (let j = 0; j < size; ++j) {
          res[i].push(x[idx(size, i, j)]);
        }
      }
      return res;
    }

    let a = new FloatsMatrix(convert(aa)), b = new FloatsMatrix(convert(ba));

    return timeit(() => a.dot(b))[0];
  }
}

Comlink.expose(MatMul);
