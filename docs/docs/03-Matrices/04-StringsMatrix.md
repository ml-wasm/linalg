---
title: StringsMatrix
---

StringsMatrix is a two dimensional array or a matrix of 32-bit integers.

The following scripts assume that you have imported the `StringsMatrix` object
from the package and set up the threads as explained in [getting started](../).

:::note

Each of these have a `R` and `C` variant. `R` variant returns a StringsVector by
applying the operation row-wise. `C` variant returns a StringsVector by applying
the operation on each column-wise.

:::

## Constructors Methods

These methods are used to create new `StringsMatrix`s.

```js
// Create a StringsMatrix from a given JavaScript array
const a = new StringsMatrix([["a", "b", "c"], ["d", "e", "f"]]);
console.log(a.data); // [["a", "b", "c"], ["d", "e", "f"]]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new StringsMatrix([["a", "b", "c"], ["d", "e", "f"]]);

// Both toJSON and data return a JavaScript array representation of the
// StringsMatrix
console.log(a.toJSON()); // [["a", "b", "c"], ["d", "e", "f"]]
console.log(a.data); // [["a", "b", "c"], ["d", "e", "f"]]

// This returns the data and metadata about the StringsMatrix
console.log(a.toString());
// [["a", "b", "c"],
//  ["d", "e", "f"]], shape=[2, 3], strides=[3, 1], layout=Cc (0x5), const ndim=2

// It returns clone of the StringsMatrix
const b = a.clone();
console.log(b.data); // [["a", "b", "c"], ["d", "e", "f"]]
```

## Utility Methods

Basic getters and setters.

```js
const x = new StringsMatrix([["a", "b", "c"], ["d", "e", "f"]]);

// Get the number of rows in the StringsMatrix
console.log(x.nrows()); // 2

// Get the number of columns in the StringsMatrix
console.log(x.ncols()); // 3

// Get the shape of the StringsMatrix
console.log(x.shape()); // [2, 3]

// Set the given value at the specified index
console.log(x.get([0, 1])); // b

// Get the value at the specified index
x.setR(1, new StringsVector(["g", "h", "i"]));
console.log(x.data);
// [["a", "b", "c"],
//  ["g", "h", "i"]]

// Swap the values at the specified indices
x.swapC(0, 1);
console.log(x.data);
// [["b", "a", "c"],
//  ["h", "g", "i"]]
```

More complex methods used to manipulate the `StringsMatrix`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append  -> appended`,
`extend  -> extended`,
`insert  -> inserted`,
`splice  -> spliced`

:::

```js
const a = new StringsMatrix([
  ['a', 'b', 'c'],
  ['d', 'e', 'f'],
]);
const b = new StringsMatrix([
  ['u', 'v', 'w'],
  ['x', 'y', 'z'],
]);

// Append an element to the StringsVector
console.log(a.appendedR(new StringsVector(['l', 'm', 'n'])).data);
// [['a', 'b', 'c'],
//  ['d', 'e', 'f'],
//  ['l', 'm', 'n']]

// Extend the StringsMatrix with another
console.log(a.extendedC(b).data);
// [['a', 'b', 'c', 'u', 'v', 'w'],
//  ['d', 'e', 'f', 'x', 'y', 'z']]

// Insert the given element at the specified index
console.log(a.insertedR(1, new StringsVector(['l', 'm', 'n'])).data);
// [['a', 'b', 'c'],
//  ['l', 'm', 'n']]
//  ['d', 'e', 'f'],

// Removes an element from the specified index
const [spliced, column] = a.splicedC(1);
console.log(spliced.data, column.data);
// [['a', 'c'],
//  ['d', 'f'],
//
// ['b', 'e']
```

## Iteration Methods

These methods allow you to perform element-wise operations on the matrix.

```js
const a = new StringsMatrix([['a', 'b'], ['c', 'd']]);

const b = a.map(x => x + 'x');
console.log(b.data); // [['ax', 'bx'], ['cx', 'dx']]

a.forEach(x => console.log(x));
// a
// b
// c
// d

a.transform(x => x + x);
console.log(a.data); // [['aa', 'bb'], ['cc', 'dd']]
```
