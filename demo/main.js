import init, {
  initThreadPool,
  IntegersVector,
  FloatsVector,
  StringsVector,
  IntegersMatrix,
  FloatsMatrix,
  StringsMatrix,
} from '@ml.wasm/linalg';

(async () => {
  // This init function sets up everything you need to use this library
  await init();

  // This sets up the concurrency
  await initThreadPool(navigator.hardwareConcurrency);

  // All your code goes here...
})();
