[![github]](https://github.com/usagi/vec-dimension-shift)&ensp;[![crates-io]](https://crates.io/crates/vec-dimension-shift)&ensp;[![docs-rs]](https://docs.rs/vec-dimension-shift)<br>
[![Build Status](https://travis-ci.org/usagi/vec-dimension-shift.svg?branch=master)](https://travis-ci.org/usagi/vec-dimension-shift)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# Vec Dimension Shift

This crate is an extension for `Vec<T>` that extends the dimension-shift features.

Note: vec-dimension-shift crate use an unsafe features in the internal details. As an alternative, [dimension_shift_buffer][] is available if you want a more flexible runtime dimension shifting or safe-only implementation.

[dimension_shift_buffer]:

## What's the "dimension shift"?

In basically,

- `Vec<f64>` --( Dimension shift, to the 2-dimension! )--> `Vec<[f64;2]>`
  - Original datum type is `f64` ≈ `[f64;1]`, 2D-shifted datum type is `[f64;2]`.
  - Original datum element type is `f64`, 2D-shifted datum element type is `f64`.
  - All datum elements were preserved.
- `Vec<[f64;2]>` --( Dimension shift, flatten! )--> `Vec<f64>`
  - Of course, it's same too.

## Implemented features

1. `VecDimensionShift2D`, `VecDimensionShift2DFlatten` and the 2..16-dimension `trait`s for `Vec<T>`.
2. `make_vec_dimension_shift_n_dimension!` macro for make your desired N-dimension `trait`s.

Note: In the default, 2D, 3D, 4D version of `VecDimensionShift?D` is enabled. Set `default-features=false` if you don't need these.

## Example

```toml
[dependencies]
vec-dimension-shift = "*"
```

### Example-1

1D -> 2D -> 1D -> 3D -> 1D, and modify an element:

```rust
use vec_dimension_shift::{
 VecDimensionShift2D,
 VecDimensionShift2DFlatten,
 VecDimensionShift3D,
 VecDimensionShift3DFlatten
};

fn d2_and_d3()
{
 let original = vec![0.0, 1.1, 2.2, 3.3, 4.4, 5.5];
 dbg!(&original);

 let mut d2_shifted = original.as_2d_array().unwrap();
 dbg!(&d2_shifted);
 assert_eq!(d2_shifted[0], [0.0, 1.1]);
 assert_eq!(d2_shifted[1], [2.2, 3.3]);
 assert_eq!(d2_shifted[2], [4.4, 5.5]);
 d2_shifted[1][1] = -1.0;

 let flatten = d2_shifted.as_flatten();
 dbg!(&flatten);

 let mut d3_shifted = flatten.as_3d_array().unwrap();
 dbg!(&d3_shifted);
 assert_eq!(d3_shifted[0], [0.0, 1.1, 2.2]);
 assert_eq!(d3_shifted[1], [-1.0, 4.4, 5.5]);
 d3_shifted[1][1] = -2.0;

 let flatten = d3_shifted.as_flatten();
 dbg!(&flatten);

 assert_eq!(flatten, vec![0.0, 1.1, 2.2, -1.0, -2.0, 5.5])
}
```

### Example-2

1. Make `trait`s just you needs (eg, 2D and 3D)
2. -> Make 1D * 12-length buffer
3. -> Shift the dimension to 2D * 6-length buffer
4. -> Shift the dimension to ( 2D * 3D ) * 2-length buffer

```rust
use vec_dimension_shift::make_vec_dimension_shift_n_dimension;

fn n_dimension_macro_generator()
{
 make_vec_dimension_shift_n_dimension! { VecDimensionShift2D, VecDimensionShift2DFlatten, as_2d_array_no_check, to_2d_array_no_check, as_2d_array, to_2d_array, as_2d_array_truncate, to_2d_array_truncate, as_2d_array_padding, to_2d_array_padding, 2 }
 make_vec_dimension_shift_n_dimension! { VecDimensionShift3D, VecDimensionShift3DFlatten, as_3d_array_no_check, to_3d_array_no_check, as_3d_array, to_3d_array, as_3d_array_truncate, to_3d_array_truncate, as_3d_array_padding, to_3d_array_padding, 3 }

 let original = vec![0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10, 11.11];
 dbg!(&original);

 let d2 = original.as_2d_array().unwrap();
 assert_eq!(d2[0], [0.0, 1.1]);
 assert_eq!(d2[1], [2.2, 3.3]);
 assert_eq!(d2[2], [4.4, 5.5]);
 assert_eq!(d2[3], [6.6, 7.7]);
 assert_eq!(d2[4], [8.8, 9.9]);
 assert_eq!(d2[5], [10.10, 11.11]);
 dbg!(&d2);

 let d3 = d2.as_3d_array().unwrap();
 assert_eq!(d3[0], [[0.0, 1.1], [2.2, 3.3], [4.4, 5.5]]);
 assert_eq!(d3[1], [[6.6, 7.7], [8.8, 9.9], [10.10, 11.11]]);
 dbg!(&d3);
}
```

## License

- [MIT](LICENSE.md)

## Author

- [USAGI.NETWORK / Usagi Ito](https://github.com/usagi/)
