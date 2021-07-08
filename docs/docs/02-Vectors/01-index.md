---
title: Vectors
---

Currently there are three supported data types for one dimensional arrays or
vectors:

- [IntegersVector](IntegersVector): Used to represent a vector of integers. The
  underlying type used is a 32-bit integer. It supports math operations but
  doesn't support complex statistical operations.
- [FloatsVector](FloatsVector): Used to represent a vector of floats. The underlying
  type used is a 64-bit floating point number. It supports math and statistical
  operations.
- [StringsVector](StringsVector): Used to represent a vector of strings. As expected, it
  doesn't support any mathematical or statistical operations.

Other types like booleans, dates, objects will probably be implemented in the
future.
