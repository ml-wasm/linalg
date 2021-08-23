import init, {
  FloatsMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  //await initThreadPool(navigator.hardwareConcurrency);
  const mat = FloatsMatrix.newWithElement(2, 3, 4);

  const vec = mat.toVector();

  const new_mat = vec.toMatrix(3, 2);
 
  console.log("Matrix");
  console.log(mat.data);
  console.log("Vector");
  console.log(vec.data);
  console.log("New mat");
  console.log(new_mat.data);
})();
