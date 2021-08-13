import init, {
  IntegersMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  //await initThreadPool(navigator.hardwareConcurrency);
  const mat = new IntegersMatrix([[1, 3, 4], [4, 7, 3]]);

  const vec = mat.toVector();
  console.log("Matrix");
  console.log(mat.data);
  console.log("Vector");
  console.log(vec.data);
})();
