import { Floats2d } from '../../../pkg/linalg.js';

export const bigTest = (start, inclusiveEnd, step, iters) => {
  const data = {
    'size': [],
    'time': [],
    'type': [],
  };
  for (let i = start; i <= inclusiveEnd; i += step) {
    try {
      for (let j = 0; j < iters; ++j) {
        const [ntime, wtime] = matrixMultiplicationTest(i, i);
        data['size'].push(i);
        data['time'].push(ntime);
        data['type'].push('naive');

        data['size'].push(i);
        data['time'].push(wtime);
        data['type'].push('wasml');
      }
      console.log(i, 'done');
    } catch (e) {
      console.error(e);
    }
  }
  console.log(data);
  console.log(JSON.stringify(data));
}

export const matmul = (iters, n) => {
  console.log(`Multiplying ${iters} ${n}x${n} matrices ->`);
  const naive = [];
  const wasml = [];
  try {
    for (let i = 0; i < iters; ++i) {
      const [ntime, wtime] = matrixMultiplicationTest(n, n);
      naive.push(ntime);
      wasml.push(wtime);
    }
  } catch (e) {
    console.error(e);
  }
  const naive_avg = naive.reduce((a, b) => a + b) / naive.length;
  const naive_max = Math.max(...naive);
  const naive_min = Math.min(...naive);
  const wasml_avg = wasml.reduce((a, b) => a + b) / wasml.length;
  const wasml_max = Math.max(...wasml);
  const wasml_min = Math.min(...wasml);

  console.table({
    Naive: {
      Maximum: naive_max,
      Minimum: naive_min,
      Average: naive_avg,
    },
    WASML: {
      Maximum: wasml_max,
      Minimum: wasml_min,
      Average: wasml_avg,
    },
  });
};

export function timeit(callback) {
  const bef = new Date();
  const ret = callback();
  const aft = new Date();

  return [ret, aft - bef];
}

export function matrixMultiplicationTest(size) {
  const a = generateMatrix(size, size);
  const b = generateMatrix(size, size);

  const x = new Floats2d(a);
  const y = new Floats2d(b);

  const [nRes, vTime] = timeit(() => vanillaMultiplication(a, b));
  const [oRes, oTime] = timeit(() => x.dot(y));

  x.free();
  y.free();
  oRes.free();

  // console.log(
  //   `Time for pure js implementation: ${vTime.toString().padStart(8)} ms`
  // );
  // console.log(
  //   `Time for our implementation:     ${oTime.toString().padStart(8)} ms`
  // );

  return [vTime, oTime];
}

function generateMatrix(m, n) {
  let mat = new Array(m);
  for (let i = 0; i < m; i++) {
    mat[i] = new Array(n);
    for (let j = 0; j < n; j++) {
      mat[i][j] = Math.random() * 100;
    }
  }
  return mat;
}

function vanillaMultiplication(a, b) {
  let m = a.length,
    n = a[0].length;
  let p = b.length,
    q = b[0].length;
  let res = new Array(m);

  for (let i = 0; i < m; i++) {
    res[i] = new Array(n);
    for (let j = 0; j < q; j++) {
      let sum = 0;
      for (let k = 0; k < n; k++) {
        sum += a[i][k] * b[k][j];
      }
      res[i][j] = sum;
    }
  }
  return res;
}
