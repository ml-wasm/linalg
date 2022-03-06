import { FloatsVector } from '@ml.wasm/linalg';
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
    let a = FloatsVector.newWithRandom(1e7);
    // console.log(a.data);

    const arr = a.data;
    const typed = Float64Array.from(a.data);
    const map = a.clone();
    const func = a.clone();
    const pfunc = a.clone();

    const op = (x: number) => x * 1234567890 + 9876543210;

    const [arrTime] = timeit(() => arr.map(op));
    const [typedTime] = timeit(() => typed.map(op));
    const [mapTime, mapReturn] = timeit(() => map.map(op));
    const [funcTime, funcReturn] = timeit(() => func.mulAdd(1234567890, 9876543210));
    const [pfuncTime, pfuncReturn] = timeit(() => pfunc.mulAdd_(1234567890, 9876543210));

    mapReturn.free();
    funcReturn.free();
    pfuncReturn.free();

    console.log("JavaScript Arrays", arrTime);
    console.log("Typed Arrays", typedTime);
    console.log("Using map", mapTime);
    console.log("Using function", funcTime);
    console.log("Using parallel function", pfuncTime);
  }
}

Comlink.expose(ScratchPad);
