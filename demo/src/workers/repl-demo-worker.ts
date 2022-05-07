import { FloatsMatrix } from '@ml.wasm/linalg';
import * as Comlink from 'comlink';
import { Array1d, Array2d, Linalg } from '../utils/types';
import { dimension, Dimension } from '../utils/dimensions';

export class ReplDemo {
  wasm!: Linalg;
  arrays: Record<string, FloatsMatrix> = {};
  async init() {
    const wasm = await import('@ml.wasm/linalg');
    const threads = navigator.hardwareConcurrency;

    await wasm.default();
    await wasm.initThreadPool(threads);

    console.log(`Inits completed (threads: ${threads})`);

    this.wasm = wasm;
  }
  createArrays(arraySizes: Record<string, Array1d>) {
    for (let [tag, size] of Object.entries(arraySizes)) {
      const matrix = FloatsMatrix.newWithRandomUniform(size, -10, 10);
      this.arrays[tag] = matrix;
    }
  }
  getArraysData(): Record<string, Array2d> {
    const arraysData: Record<string, Array2d> = {};
    for (let [tag, matrix] of Object.entries(this.arrays)) {
      arraysData[tag] = matrix.data;
    }
    return arraysData;
  }
  performOperation(functionBody: string): Record<string, Array2d> {
    const tags: string[] = [], matrices: FloatsMatrix[] = [];
    for (let [tag, matrix] of Object.entries(this.arrays)) {
      tags.push(tag);
      matrices.push(matrix.clone());
    }
    const result = new Function(...tags, `return ${functionBody}`)(...matrices);
    console.log(result.data);

    const arrays: Record<string, Array2d> = {};
    switch (dimension(typeof result == 'number' ? result : result.data)) {
      case Dimension.None:
        for (let i = 0; i < tags.length; ++i) {
          arrays[tags[i]] = matrices[i].data;
        }
        break;
      case Dimension.Zero:
        arrays['result'] = [[result]];
        break;
      case Dimension.One:
        arrays['result'] = [result.data];
        break;
      case Dimension.Two:
        arrays['result'] = result.data;
        break;
    }

    return arrays;
  }
}

Comlink.expose(ReplDemo);
