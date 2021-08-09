import init, {
  StringsMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  //await initThreadPool(navigator.hardwareConcurrency);
  const mat = new StringsMatrix([["ads", "dsf"], ["df", "fd"]]);
 
  mat.mapRows((x) => {
    console.log(x.data);
  });

  mat.mapCols((x) => {
    console.log(x.data);
  });

})();
