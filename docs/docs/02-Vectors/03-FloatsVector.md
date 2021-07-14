---
title: FloatsVector
---

FloatsVector is an one dimensional array or a vector of 64-bit floats.

The following scripts assume that you have imported the `FloatsVector` object
from the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `FloatsVector`s.

```js
// Create a FloatsVector from a given JavaScript array
const a = new FloatsVector([1, 2, 3, 4, 5]);
console.log(a.data); // [1, 2, 3, 4, 5]

// Create a FloatsVector filled with zeros of the specified length
const b = FloatsVector.newWithZeros(5);
console.log(b.data); // [0, 0, 0, 0, 0]

// Create a FloatsVector filled with ones of the specified length
const c = FloatsVector.newWithOnes(5);
console.log(c.data); // [1, 1, 1, 1, 1]

// Create a FloatsVector filled with given element of the specified length
const d = FloatsVector.newWithElement(5, 2);
console.log(d.data); // [2, 2, 2, 2, 2]

// Create a FloatsVector of specified length calling the given function
// without any parameters at every element
const e = FloatsVector.newWithSimpleFunc(3, () => Math.random() * 10);
console.log(e.data); // [1.6528487232156874, 3.3231458607562137, 8.349146447113004]

// Create a FloatsVector of specified length calling the given function with
// the index as the only parameter for every element
const f = FloatsVector.newWithFunc(5, i => i * i);
console.log(f.data); // [0, 1, 4, 9, 16]

// Create a FloatsVector filled with n evenly spaced elements between two points
const g = FloatsVector.newWithLinspace(0, 1, 5);
console.log(g.data); // [0, 0.25, 0.5, 0.75, 1]

// Create a FloatsVector with elements from start to step incrementing by step
const h = FloatsVector.newWithRange(0, 0.75, 0.25);
console.log(h.data); // [0, 0.25, 0.5]

// Create a FloatsVector with n logarithmically spaced elements from base^start
// to base^end
const i = FloatsVector.newWithLogspace(2, 0, 10, 5);
console.log(i.data); // [2, 4, 8, 16]

// Create a FloatsVector with n geometrically spaced elements from start to end
const j = FloatsVector.newWithGeomspace(1, 5, 3);
console.log(j.data); // [1, 1.709975946676697, 2.924017738212866, 5.000000000000001]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new FloatsVector([1, 2, 3]);

// Both toJSON and data return a JavaScript array representation of the
// FloatsVector
console.log(a.toJSON()); // [1, 2, 3]
console.log(a.data); // [1, 2, 3]

// This returns the data and metadata about the FloatsVector
console.log(a.toString());
// "[1, 2, 3], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1"

// It returns clone of the FloatsVector
const b = a.clone();
console.log(b.data); // [1, 2, 3]
```

## Utility Methods

Basic getters and setters.

```js
const x = new FloatsVector([1, 2, 3]);

// Get the length of the array
console.log(x.len()); // 3

// Get the shape of the array
console.log(x.shape()); // [3]

// Set the given value at the specified index
x.set(1, 7);

// Get the value at the specified index
console.log(x.get(1)); // 7

// Swap the values at the specified indices
x.swap(0, 2);
console.log(x.data); // [3, 7, 1]
```

More complex methods used to manipulate the `FloatsVector`. Each of these
methods has two versions. The "pure" version returns the result of performing
the operation while the "impure" version actually changes the array.

| Pure    | Impure   |
| ------- | -------- |
| reverse | reversed |
| append  | appended |
| extend  | extended |
| insert  | inserted |
| splice  | spliced  |

```js
const a = new FloatsVector([0.1, 0.2, 0.3]);
const b = new FloatsVector([0.4, 0.5, 0.6]);

// Reverse the FloatsVector
console.log(a.reversed().data);
// [0.3, 0.2, 0.1]

// Append an element to the FloatsVector
console.log(a.appended(0.7).data);
// [0.1, 0.2, 0.3, 0.7]

// Extend the FloatsVector with another
console.log(a.extended(b).data);
// [0.1, 0.2, 0.3, 0.4, 0.5, 0.6]

// Insert the given element at the specified index
console.log(a.inserted(1, 0.7).data);
// [0.1, 0.7, 0.2, 0.3]

// Removes an element from the specified index
const [spliced, element] = a.spliced(1);
console.log(spliced.data, element);
// [0.1, 0.3] 0.2
```

## Math Methods

Methods to perform simple mathematical operations on the array.

```js
const a = new FloatsVector([0.1, 0.2, 0.3]);
const b = new FloatsVector([0.4, 0.5, 0.6]);

// Perform element-wise addition of two FloatsVectors
console.log(a.add(b).data); // [0.5, 0.7, 0.8999999999999999]

// Perform element-wise subtraction of two FloatsVectors
console.log(a.sub(b).data); // [-0.30000000000000004, -0.3, -0.3]

// Perform element-wise multiplication of two FloatsVectors
console.log(a.mul(b).data); // [0.04000000000000001, 0.1, 0.18]

// Perform element-wise division of two FloatsVectors
console.log(b.div(a).data); // [4, 2.5, 2]

// Return the addition or product of the FloatsVector
console.log(a.sum()); // 0.6000000000000001
console.log(b.product()); // 0.12

// Efficiently perform in-place element-wise scaled addition of two FloatsVectors
a.scaledAdd(2, b);
console.log(a.data); // [0.9, 1.2, 1.5]
```

## Statistical Methods

Methods to perform basic statistical operations.

```js
const a = new FloatsVector([0.1, 0.2, 0.3]);

// Return the minimum element in the array
console.log(a.min()); // 0.1

// Return the minimum element in the array
console.log(a.max()); // 0.3

// Return the mean of all the elements in the array
console.log(a.mean()); // 0.20000000000000004

// Return the standard deviation of the array
console.log(a.std(0)); // 0.0816496580927726

// Return the variance of the array
console.log(a.var(1)); // 0.009999999999999997
```
