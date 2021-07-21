import init, { initThreadPool } from '../../../pkg/linalg.js';

(async () => {
  await init();
  await initThreadPool(navigator.hardwareConcurrency);

  test();
})();

const test = () => {};
