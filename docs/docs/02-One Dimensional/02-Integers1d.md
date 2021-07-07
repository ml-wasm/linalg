---
title: Integers1d
---

Integers1d is a one dimensional array or a vector of 32-bit integers.

The following scripts assume that you have imported the `Integers1d` object from
the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `Integer1d`s.

```js
// Create an Integers1d from a given JavaScript array
const a = new Integers1d([1, 2, 3, 4, 5]);
console.log(a.data); // [1, 2, 3, 4, 5]

// Create an Integers1d filled with zeros of the specified length
const b = Integers1d.newWithZeros(5);
console.log(b.data); // [0, 0, 0, 0, 0]

// Create an Integers1d filled with ones of the specified length
const c = Integers1d.newWithOnes(5);
console.log(c.data); // [1, 1, 1, 1, 1]

// Create an Integers1d filled with given element of the specified length
const d = Integers1d.newWithElement(5, 2);
console.log(d.data); // [2, 2, 2, 2, 2]

// Create an Integers1d of specified length calling the given function without
// any parameters at every element
const e = Integers1d.newWithSimpleFunc(5, () => Math.floor(Math.random() * 10));
console.log(e.data); // [5, 2, 3, 8, 1]

// Create an Integers1d of specified length calling the given function with the
// index as the only parameter for every element
const f = Integers1d.newWithFunc(5, i => i * i);
console.log(f.data); // [0, 1, 4, 9, 16]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new Integers1d([1, 2, 3]);

// Both toJSON and data return a JavaScript array representation of the Integers1d
console.log(a.toJSON()); // [1, 2, 3]
console.log(a.data); // [1, 2, 3]

// This returns the data and metadata about the Integers1d
console.log(a.toString());
// "[1, 2, 3], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1"

// It returns clone of the Integers1d
const b = a.clone();
console.log(b.data); // [1, 2, 3]
```

## Utility Methods

Basic getter and setters.

```js
const x = new Integers1d([1, 2, 3]);

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

More complex methods used to manipulate the `Integers1d`. Each of these methods
has two versions. The "pure" version returns the result of performing the
operation while the "impure" version actually changes the array.

| Pure    | Impure   |
| ------- | -------- |
| reverse | reversed |
| append  | appended |
| extend  | extended |
| insert  | inserted |
| splice  | spliced  |

```js
const a = new Integers1d([1, 2, 3]);
const b = new Integers1d([4, 5, 6]);

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
```
