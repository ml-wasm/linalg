---
title: FloatsMatrix
---

FloatsMatrix is an two dimensional array or a matrix of 64-bit floats.

The following scripts assume that you have imported the `FloatsMatrix` object
from the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `FloatsMatrix`s.

```js
// Create a FloatsMatrix from a given JavaScript array
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
console.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);

// Both toJSON and data return a JavaScript array representation of the
// FloatsMatrix
console.log(a.toJSON()); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
console.log(a.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]

// This returns the data and metadata about the FloatsMatrix
console.log(a.toString());
// "[[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]], shape=[2, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2"

// It returns clone of the FloatsMatrix
const b = a.clone();
console.log(b.data); // [[0.1, 0.2, 0.3], [0.5, 0.6, 0.7]]
```

## Utility Methods

Basic getters and setters.

:::note

`get`, `set` and `swap` have a `R`, `C` and a normal variant. `R` variant
applies the operation to the specified row(s). `C` variant applies the operation
to the specified column(s).

:::

```js
const x = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);

// Get the number of rows in the FloatsMatrix
console.log(x.nrows()); // 2

// Get the number of columns in the FloatsMatrix
console.log(x.ncols()); // 3

// Get the shape of the FloatsMatrix
console.log(x.shape()); // [2, 3]

// Set the given value at the specified index
console.log(x.get([0, 1])); // 0.2

// Get the value at the specified index
x.setR(1, new FloatsVector([0.8, 0.9, 1.0]));
console.log(x.data);
// [[0.1, 0.2, 0.3],
//  [0.9, 0.8, 1]]

// Swap the values at the specified indices
x.swapC(0, 1);
console.log(x.data);
// [[0.1, 0.2, 0.3],
//  [0.9, 0.8, 1]]
```

More complex methods used to manipulate the `FloatsMatrix`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append -> appended`,
`extend -> extended`,
`insert -> inserted`,
`splice -> spliced`

:::

:::note

Each of these has a `R` and `C` variant. `R` variant returns a FloatsVector by
applying the operation row-wise. `C` variant returns a FloatsVector by applying
the operation on each column-wise.

:::

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
const b = new FloatsMatrix([
  [0.8, 0.9, 1.0],
  [1.1, 1.2, 1.3],
]);

// Append an element to the FloatsMatrix
console.log(a.appendedR(new FloatsVector([1.4, 1.5, 1.6])).data);
// [[0.1, 0.2, 0.3],
//  [0.5, 0.6, 0.7],
//  [1.4, 1.5, 1.6]]

// Extend the FloatsMatrix with another
console.log(a.extendedC(b).data);
// [[0.1, 0.2, 0.3, 0.8, 0.9, 1.0],
//  [0.5, 0.6, 0.7, 1.1, 1.2, 1.3]]

// Insert the given element at the specified index
console.log(a.insertedR(1, new FloatsVector([1.4, 1.5, 1.6])).data);
// [[0.1, 0.2, 0.3],
//  [1.4, 1.5, 1.6],
//  [0.5, 0.6, 0.7]]

// Removes an element from the specified index
const [spliced, column] = a.splicedC(1);
console.log(spliced.data, column.data);
// [[0.1, 0.3],
//  [0.5, 0.7]]
//
// [0.2, 0.6]
```

## Iteration Methods

These methods allow you to perform element-wise operations on the matrix.

```js
const a = new FloatsMatrix([[0.1, 0.2], [0.3, 0.4]]);

const b = a.map(x => x * 3);
console.log(b.data);
// [[0.30000000000000004, 0.6000000000000001],
//  [0.8999999999999999, 1.2000000000000002]]

a.forEach(x => console.log(x));
// 0.1
// 0.2
// 0.3
// 0.4

a.transform(x => x * x);
console.log(a.data);
// [[0.010000000000000002, 0.04000000000000001],
//  [0.09, 0.16000000000000003]],
```

## Math Methods

Methods to perform simple mathematical operations on the array.

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.5, 0.6, 0.7],
]);
const b = new FloatsMatrix([
  [0.8, 0.9, 1.0],
  [1.1, 1.2, 1.6],
]);

// Perform element-wise addition of two FloatsMatrices
console.log(a.add(b).data);
// [[0.9, 1.1, 1.3],
//  [1.6, 1.7999999999999998, 2.3]]

// Perform element-wise subtraction of two FloatsMatrices
console.log(a.sub(b).data);
// [[0.9, 1.1, 1.3],
//  [1.6, 1.7999999999999998, 2.3]]

// Perform element-wise multiplication of two FloatsMatrices
console.log(a.mul(b).data);
// [[0.08000000000000002, 0.18000000000000002, 0.3],
//  [0.55, 0.72, 1.1199999999999999]]

// Perform element-wise division of two FloatsMatrices
console.log(b.div(a).data);
// [[8, 4.5, 3.3333333333333335],
//  [2.2, 2, 2.285714285714286]]

// Transposes the FloatsMatrix. That is it exchanges the rows and columns.
// a.tranposed() will return the result of performing the operations
b.transpose();
console.log(b.data);
// [[0.8, 1.1],
//  [0.9, 1.2],
//  [1, 1.6]]

// Performs dot product of the two FloatsMatrices
console.log(a.dot(b).data);
// [[0.56, 0.83],
//  [1.6400000000000001, 2.3899999999999997]]

// Return the addition or product of the FloatsMatrices
console.log(a.sum()); // 2.4000000000000004
console.log(b.product()); // 1.5206400000000002

// Efficiently perform in-place element-wise scaled addition of two FloatsMatrices
a.scaledAdd(2, b.transposed());
console.log(a.data);
// [[1.7000000000000002, 2, 2.3],
//  [2.7, 3, 3.9000000000000004]]
```

## Statistical Methods

Methods to perform basic statistical operations.

:::note

All of these methods has a `R`, `C` and normal variant. `R` variant returns a
FloatsVector by applying the operation on each row. `C` variant returns a
FloatsVector by applying the operation on each column.

:::

```js
const a = new FloatsMatrix([
  [0.1, 0.2, 0.3],
  [0.4, 0.5, 0.6],
]);

// Return the minimum element in the array
console.log(a.min()); // 0.1

// Return the minimum element in each row
console.log(a.maxR().data); // [0.3, 0.6]

// Returns the mean of each column
console.log(a.meanC().data); // [0.25, 0.35, 0.44999999999999996]

// Return the standard deviation in each row
console.log(a.stdR(0).data); // [0.0816496580927726, 0.08164965809277258]

// Return the variance of the array
console.log(a.varC(1).data); // [0.04500000000000001, 0.045000000000000005, 0.045000000000000005]
```
