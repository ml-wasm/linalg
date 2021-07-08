---
title: Matrices
---

Currently there are three supported data types for two dimensional arrays or
matrices:

- [IntegersMatrix](IntegersMatrix): Used to represent a matrix of
  integers. The underlying type used is a 32-bit integer. It supports math
  operations but doesn't support complex statistical operations.
- [FloatsMatrix](FloatsMatrix): Used to represent a matrix of floats. The
  underlying type used is a 64-bit floating point number. It supports math and
  statistical operations.
- [StringsMatrix](StringsMatrix): Used to represent a matrix of strings.
  As expected, it doesn't support any mathematical or statistical operations.

Other types like booleans, dates, objects will probably be implemented in the
future.
