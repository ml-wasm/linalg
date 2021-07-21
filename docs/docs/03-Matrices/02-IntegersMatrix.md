---
title: IntegersMatrix
---

IntegersMatrix is an two dimensional array or a matrix of 32-bit integers.

The following scripts assume that you have imported the `IntegersMatrix` object
from the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `IntegersMatrix`s.

```js
// Create a IntegersMatrix from a given JavaScript array
const a = new IntegersMatrix([
  [1, 2, 3],
  [5, 6, 7],
]);
console.log(a.data); // [[1, 2, 3], [5, 6, 7]]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new IntegersMatrix([
  [1, 2, 3],
  [5, 6, 7],
]);

// Both toJSON and data return a JavaScript array representation of the
// IntegersMatrix
console.log(a.toJSON()); // [[1, 2, 3], [5, 6, 7]]
console.log(a.data); // [[1, 2, 3], [5, 6, 7]]

// This returns the data and metadata about the IntegersMatrix
console.log(a.toString());
// "[[1, 2, 3], [5, 6, 7]], shape=[2, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2"

// It returns clone of the IntegersMatrix
const b = a.clone();
console.log(b.data); // [[1, 2, 3], [5, 6, 7]]
```

## Utility Methods

Basic getters and setters.

:::note

`get`, `set` and `swap` have a `R`, `C` and a normal variant. `R` variant
applies the operation to the specified row(s). `C` variant applies the operation
to the specified column(s).

:::

```js
const x = new IntegersMatrix([
  [1, 2, 3],
  [5, 6, 7],
]);

// Get the number of rows in the IntegersMatrix
console.log(x.nrows()); // 2

// Get the number of columns in the IntegersMatrix
console.log(x.ncols()); // 3

// Get the shape of the IntegersMatrix
console.log(x.shape()); // [2, 3]

// Set the given value at the specified index
console.log(x.get([0, 1])); // 2

// Get the value at the specified index
x.setR(1, new IntegersVector([8, 9, 10]));
console.log(x.data);
// [[1, 2, 3],
//  [8, 9, 10]]

// Swap the values at the specified indices
x.swapC(0, 1);
console.log(x.data);
// [[2, 1, 3],
//  [9, 8, 10]]
```

More complex methods used to manipulate the `IntegersMatrix`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append -> appended`,
`extend -> extended`,
`insert -> inserted`,
`splice -> spliced`

:::

:::note

Each of these has a `R` and `C` variant. `R` variant returns a IntegersVector by
applying the operation row-wise. `C` variant returns a IntegersVector by applying
the operation on each column-wise.

:::

```js
const a = new IntegersMatrix([
  [1, 2, 3],
  [5, 6, 7],
]);
const b = new IntegersMatrix([
  [8, 9, 10],
  [11, 12, 13],
]);

// Append an element to the IntegersMatrix
console.log(a.appendedR(new IntegersVector([14, 15, 16])).data);
// [[1, 2, 3],
//  [5, 6, 7],
//  [14, 15, 16]]

// Extend the IntegersMatrix with another
console.log(a.extendedC(b).data);
// [[1, 2, 3, 8, 9, 10],
//  [5, 6, 7, 11, 12, 13]]

// Insert the given element at the specified index
console.log(a.insertedR(1, new IntegersVector([14, 15, 16])).data);
// [[1, 2, 3],
//  [14, 15, 16],
//  [5, 6, 7]]

// Removes an element from the specified index
const [spliced, column] = a.splicedC(1);
console.log(spliced.data, column.data);
// [[1, 3],
//  [5, 7]]
//
// [2, 6]
```

## Iteration Methods

These methods allow you to perform element-wise operations on the matrix.

```js
const a = new IntegersMatrix([[1, 2], [3, 4]]);

const b = a.map(x => x * 3);
console.log(b.data); // [[3, 6], [9, 12]]
a.forEach(x => console.log(x));
// 1
// 2
// 3
// 4

a.transform(x => x * x);
console.log(a.data); // [[1, 4], [9, 16]]
```

## Math Methods

Methods to perform simple mathematical operations on the array.

```js
const a = new IntegersMatrix([
  [1, 2, 3],
  [5, 6, 7],
]);
const b = new IntegersMatrix([
  [8, 9, 1],
  [11, 12, 16],
]);

// Perform element-wise addition of two IntegersMatrices
console.log(a.add(b).data);
// [[9, 11, 4]
//  [16, 18, 23]]

// Perform element-wise subtraction of two IntegersMatrices
console.log(a.sub(b).data);
// [[-7, -7, 2],
//  [-6, -6, -9]]

// Perform element-wise multiplication of two IntegersMatrices
console.log(a.mul(b).data);
// [[8, 18, 3],
//  [55, 72, 112]]

// Perform element-wise division of two IntegersMatrices
console.log(b.div(a).data);
// [[8, 4, 0],
//  [2, 2, 2]]

// Transposes the IntegersMatrix. That is it exchanges the rows and columns.
// a.tranposed() will return the result of performing the operations
b.transpose();
console.log(b.data);
// [[8, 11],
//  [9, 12],
//  [1, 16]]

// Performs dot product of the two IntegersMatrices
console.log(a.dot(b).data);
// [[29, 83],
//  [101, 239]]

// Return the addition or product of the IntegersMatrices
console.log(a.sum()); // 24
console.log(b.product()); // 152064

// Efficiently perform in-place element-wise scaled addition of two IntegersMatrices
a.scaledAdd(2, b.transposed());
console.log(a.data);
// [[17, 20, 5],
//  [27, 30, 39]]
```

## Statistical Methods

Methods to perform basic statistical operations.

:::note

All of these methods has a `R`, `C` and normal variant. `R` variant returns a
IntegersVector by applying the operation on each row. `C` variant returns a
IntegersVector by applying the operation on each column.

:::

```js
const a = new IntegersMatrix([
  [1, 2, 3],
  [4, 5, 6],
]);

// Return the minimum element in the array
console.log(a.min()); // 0.2

// Return the minimum element in each row
console.log(a.maxR().data); // [3, 6]

// Returns the mean of each column
console.log(a.meanC().data); // [2, 3, 4]
```
