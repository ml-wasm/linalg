import init, { initThreadPool, StringsVector } from '../../../pkg/linalg.js';

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

  // console.group(
  //   '%cONE DIMENSIONAL',
  //   'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  // );
  // one_dimensional_floats();
  // console.groupEnd();

  // console.group(
  //   '%cTWO DIMENSIONAL',
  //   'color: white; background-color: darkblue; padding: 5px 10px; border-radius: 5px'
  // );
  // two_dimensional_floats();

  // console.groupEnd();
})();

const test = () => {
const a = new StringsVector(["a", "b", "c"]);
const b = new StringsVector(["d", "e", "f"]);

// Reverse the StringsVector
console.log(a.reversed().data);
// ["c", "b", "a"]

// Append an element to the StringsVector
console.log(a.appended("g").data);
// ["a", "b", "c", "g"]

// Extend the StringsVector with another
console.log(a.extended(b).data);
// ["a", "b", "c", "d", "e", "f"]

// Insert the given element at the specified index
console.log(a.inserted(1, "g").data);
// ["a", "g", "b", "c"]

// Removes an element from the specified index
const [spliced, element] = a.spliced(1);
console.log(spliced.data, element);
// ["a", "c"] "b"

};
