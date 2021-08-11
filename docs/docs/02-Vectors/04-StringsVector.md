---
title: StringsVector
---

StringsVector is an one dimensional array or a vector of strings.

The following scripts assume that you have imported the `StringsVector` object
from the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `StringsVector`s.

```js
// Create an StringsVector from a given JavaScript array
const a = new StringsVector(['a', 'b', 'c']);
console.log(a.data); // ["a", "b", "c"]

// Create an StringsVector of specified length calling the given function
// without any parameters at every element
const b = StringsVector.newWithSimpleFunc(5, () =>
  'a'.repeat(Math.floor(Math.random() * 5))
);
console.log(b.data); // ["aaa", "", "aaaa", "", ""]

// Create an StringsVector of specified length calling the given function with
// the index as the only parameter for every element
const c = StringsVector.newWithFunc(5, i => 'b'.repeat(i));
console.log(c.data); // ["", "b", "bb", "bbb", "bbbb"]
```

## Interop Methods

Some handy methods to work with the array.

```js
const a = new StringsVector(['a', 'b', 'c']);

// Both toJSON and data return a JavaScript array representation of the
// StringsVector
console.log(a.toJSON()); // ["a", "b", "c"]
console.log(a.data); // ["a", "b", "c"]

// This returns the data and metadata about the StringsVector
console.log(a.toString());
// "[1, 2, 3], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1"

// It returns clone of the StringsVector
const b = a.clone();
console.log(b.data); // ["a", "b", "c"]
```

## Utility Methods

Basic getters and setters.

```js
const x = new StringsVector(['a', 'b', 'c']);

// Get the length of the array
console.log(x.len()); // 3

// Get the shape of the array
console.log(x.shape()); // [3]

// Set the given value at the specified index
x.set(1, 'e');

// Get the value at the specified index
console.log(x.get(1)); // "e"

// Swap the values at the specified indices
x.swap(0, 2);
console.log(x.data); // ["c", "e", "a"]
```

More complex methods used to manipulate the `StringsVector`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append -> appended`,
`extend -> extended`,
`insert -> inserted`,
`splice -> spliced`

:::

```js
const a = new StringsVector(['a', 'b', 'c']);
const b = new StringsVector(['d', 'e', 'f']);

// Reverse the StringsVector
console.log(a.reversed().data);
// ["c", "b", "a"]

// Append an element to the StringsVector
console.log(a.appended('g').data);
// ["a", "b", "c", "g"]

// Extend the StringsVector with another
console.log(a.extended(b).data);
// ["a", "b", "c", "d", "e", "f"]

// Insert the given element at the specified index
console.log(a.inserted(1, 'g').data);
// ["a", "g", "b", "c"]

// Removes an element from the specified index
const [spliced, element] = a.spliced(1);
console.log(spliced.data, element);
// ["a", "c"] "b"
```

## Iteration Methods

These methods allow you to perform element-wise operations on the vector.

```js
const a = new StringsVector(['a', 'b', 'c']);

const b = a.map(x => x + 'x');
console.log(b.data); // ["ax", "bx", "cx"]

a.forEach(x => console.log(x));
// a
// b
// c

a.transform(x => x + x);
console.log(a.data); // ["aa", "bb", "cc"]
```
