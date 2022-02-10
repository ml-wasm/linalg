export function timeit<R>(cb: () => R): [number, R] {
  const start = performance.now();
  const result = cb();
  const end = performance.now();

  return [end - start, result];
}

