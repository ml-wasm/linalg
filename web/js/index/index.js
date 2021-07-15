import init, { initThreadPool, FloatsVector, FloatsMatrix } from '../../../pkg/linalg.js';

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
	const a = new FloatsMatrix([[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]);
	const b = new FloatsMatrix([[0.8, 0.9, 1.0], [1.1, 1.2, 1.3]]);

	// Append an element to the FloatsMatrix
	console.log(a.appendedR(new FloatsVector([1.4, 1.5, 1.6])).data);
	// [[0.1, 0.2, 0.3],
	//  [0.5, 0.6, 0.7],
	//  [1.4, 1.5, 1.6]]

	// Extend the FloatsMatrix with another
	console.log(a.extendedC(b).data);
	// [[0.1, 0.2, 0.3],
	//  [0.5, 0.6, 0.7],
	//  [0.8, 0.9, 1.0],
	//  [1.1, 1.2, 1.3]]

	// Insert the given element at the specified index
	console.log(a.insertedR(1, new FloatsVector([1.4, 1.5, 1.6])).data);
	// [[0.1, 0.2, 0.3],
	//  [1.4, 1.5, 1.6],
	//  [0.5, 0.6, 0.7]]

	// Removes an element from the specified index
	const [spliced, column] = a.splicedC(1);
	console.log(spliced.data, column.data);
	// [[0.1, 0.3],
	// [0.5, 0.7]]
	//
	// [0.2, 0.6]
};
