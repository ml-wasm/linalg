---
title: FloatsVector
---

FloatsVector is an one dimensional array or a vector of 64-bit floats.

The following scripts assume that you have imported the `FloatsVector` object
from the package and set up the threads as explained in [getting started](../).

## Constructors Methods

These methods are used to create new `FloatsVector`s.

:::note

Random constructors are in the [random section](#random-methods).

:::

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

More complex methods used to manipulate the `FloatsVector`.

:::note

Each of these methods has two versions. The "pure" version returns the result of
performing the operation while the "impure" version actually changes the array.

`append -> appended`,
`extend -> extended`,
`insert -> inserted`,
`splice -> spliced`

:::

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

## Iteration Methods

These methods allow you to perform element-wise operations on the vector.

```js
const a = new FloatsVector([0.1, 0.2, 0.3]);

const b = a.map(x => x * 3);
console.log(b.data); // [0.30000000000000004, 0.6000000000000001, 0.8999999999999999]

a.forEach(x => console.log(x));
// 0.1
// 0.2
// 0.3

a.transform(x => x * x);
console.log(a.data); // [0.010000000000000002, 0.04000000000000001, 0.09]
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

## Random Methods

Methods that involve randomness.

```js
const a = FloatsVector.newWithRandom(5);
console.log(a.data);
// [0.34099948653292644, 0.5917974289682977, 0.6106158620616635, 0.23569474303579374, 0.5282766210914832]

const b = FloatsVector.newWithRandomBeta(5, 2, 5);
console.log(b.data);
// [0.23096671212947778, 0.3534113458857726, 0.34475362808214677, 0.3933605194453189, 0.4257499695718641]

const c = FloatsVector.newWithRandomCauchy(5, 2, 5);
console.log(c.data);
// [-10.259169814528763, 2.2570224421228415, -0.5143187859414864, -3.566078787268461, 1.8881158135056801]

const d = FloatsVector.newWithRandomChiSquared(5, 11);
console.log(d.data);
// [12.540935152190382, 11.870956625621746, 17.77216830488515, 10.783752732928962, 10.664649797233045]

const e = FloatsVector.newWithRandomExp(5, 10);
console.log(e.data);
// [0.1582298816595742, 0.15597280990902898, 0.026172146240797518, 0.033239085350025364, 0.07938556756009543]

const f = FloatsVector.newWithRandomExp1(5);
console.log(f.data);
// [0.3786935465784936, 0.324525176947993, 1.0327496064187238, 0.08707228389123019, 1.4594054734003976]

const g = FloatsVector.newWithRandomFisher(5, 2, 32);
console.log(g.data);
// [0.3074038581481312, 2.1847693566847393, 0.2276554651866491, 0.1163362059260453, 0.6359674157651448]

const h = FloatsVector.newWithRandomGamma(5, 2, 5);
console.log(h.data);
// [13.64534682579972, 13.18225900805788, 10.813782133484146, 12.138039285872178, 5.944892095380422]

const i = FloatsVector.newWithRandomInverseGaussian(5, 2, 5);
console.log(i.data);
// [2.271010640745854, 2.033124138574522, 3.913079484773854, 1.505314818081629, 0.6226851876937998]

const j = FloatsVector.newWithRandomLogNormal(5, 2, 3);
console.log(j.data);
// [4.657210763636486, 66.24632860255005, 20.613278570556297, 10492.633098375172, 39.12513559580575]

const k = FloatsVector.newWithRandomNormal(5, 2, 3);
console.log(k.data);
// [0.39029435201635665, 3.27261609644719, 5.0629281791580505, 0.27925119369574114, 5.240239044741968]

const l = FloatsVector.newWithRandomNormalInverseGaussian(5, 3, 2);
console.log(l.data);
// [1.2097230563868009, 0.9542971768376568, 0.12535857990674565, 0.9559384049858257, 1.0198942781079443]

const m = FloatsVector.newWithRandomOpen01(5);
console.log(m.data);
// [0.14561860008297822, 0.2078389740691543, 0.518045179245913, 0.5737861058411688, 0.7382741221752666]

const n = FloatsVector.newWithRandomOpenClosed01(5);
console.log(n.data);
// [0.9188955843628368, 0.319868332534298, 0.44071331349815046, 0.3528910186300348, 0.8702363940864016]

const o = FloatsVector.newWithRandomPERT(5, 0, 5, 2.5);
console.log(o.data);
// [0.22585318866399032, 3.600397861643679, 0.9194109223130829, 2.2344645171396156, 1.9741550812422148]

const p = FloatsVector.newWithRandomPERTwithShape(5, 0, 5, 2.5, 3);
console.log(p.data);
// [3.9604520876027465, 4.088791843088483, 1.5617571788323033, 3.436157392852803, 2.0772133687604]

const r = FloatsVector.newWithRandomPareto(5, 1, 2);
console.log(r.data);
// [1.1307878092838028, 1.0069139977625745, 1.1074903396292302, 1.060290197461119, 1.0271930755716328]

const s = FloatsVector.newWithRandomPoisson(5, 2.0);
console.log(s.data);
// [1, 2, 1, 1, 1]

const t = FloatsVector.newWithRandomStandardNormal(5);
console.log(t.data);
// [-1.174592862530223, -1.6549897461536043, -0.39595930094841264, 0.6132371623244752, -0.7785965936342404]

const u = FloatsVector.newWithRandomStudentT(5, 11);
console.log(u.data);
// [-0.5892888945409621, 0.058722742800470386, 0.2806955947923983, 1.6907601263659682, 1.6528920846027841]

const v = FloatsVector.newWithRandomTriangular(5, 0, 5, 2.5);
console.log(v.data);
// [1.1120111694995478, 4.0989948710773065, 2.802263571151345, 1.0294591379745555, 0.9801948864580505]

const w = FloatsVector.newWithRandomUniform(5, -5, 5);
console.log(w.data);
// [1.1120111694995478, 4.0989948710773065, 2.802263571151345, 1.0294591379745555, 0.9801948864580505]

const x = FloatsVector.newWithRandomWeibull(5, 1, 10);
console.log(x.data);
// [0.6117699677190211, 0.8563173903147892, 0.8959745267683338, 1.1108808965201318, 1.092721654565621]

const aa = a.sample(3);
console.log(aa.data);
// [0.02612549959263477, 0.31168488926149907, 0.77341724202506]
```
