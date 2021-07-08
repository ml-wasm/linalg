import init, { initThreadPool, IntegersVector } from '../../../pkg/linalg.js';

// import { bigTest } from './matmul.js';
import { one_dimensional_floats } from './one.js';
import { two_dimensional_floats } from './two.js';

(async () => {
  await init();
  await initThreadPool(navigator.hardwareConcurrency);

  // console.group(
  //     '%cMATRIX MULTPLICATION TEST',
  //     'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  // );
  // bigTest(100, 1000, 10, 10);
  // console.groupEnd();

  test();

  console.group(
    '%cONE DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  one_dimensional_floats();
  console.groupEnd();

  console.group(
    '%cTWO DIMENSIONAL',
    'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  );
  two_dimensional_floats();

  console.groupEnd();
})();

const test = () => {
  const a = new IntegersVector([1, 2, 3]);
  const b = new IntegersVector([4, 5, 6]);

  // Reverse the Integers1d
  console.log(a.reversed().data);
  // [3, 2, 1]

  // Append an element to the Integers1d
  console.log(a.appended(7).data);
  // [1, 2, 3, 7]

  // Extend the Integers1d with another
  console.log(a.extended(b).data);
  // [1, 2, 3, 4, 5, 6]

  // Insert the given element at the specified index
  console.log(a.inserted(1, 7).data);
  // [1, 7, 2, 3]

  // Removes an element from the specified index
  const [spliced, element] = a.spliced(1);
  console.log(spliced.data, element);
  // [1, 3] 2
};
