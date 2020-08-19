//! [![github]](https://github.com/usagi/vec-dimension-shift)&ensp;[![crates-io]](https://crates.io/crates/vec-dimension-shift)&ensp;[![docs-rs]](https://docs.rs/vec-dimension-shift)<br>
//! [![Build Status](https://travis-ci.org/usagi/vec-dimension-shift.svg?branch=master)](https://travis-ci.org/usagi/vec-dimension-shift)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! # Vec Dimension Shift
//!
//! This crate is an extension for `Vec<T>` that extends the dimension-shift features.
//!
//! Note: vec-dimension-shift crate use an unsafe features in the internal details. As an alternative, [dimension_shift_buffer][] is available if you want a more flexible runtime dimension shifting or safe-only implementation.
//!
//! [dimension_shift_buffer]:
//!
//! ## What's the "dimension shift"?
//!
//! In basically,
//!
//! - `Vec<f64>` --( Dimension shift, to the 2-dimension! )--> `Vec<[f64;2]>`
//!   - Original datum type is `f64` ≈ `[f64;1]`, 2D-shifted datum type is `[f64;2]`.
//!   - Original datum element type is `f64`, 2D-shifted datum element type is `f64`.
//!   - All datum elements were preserved.
//! - `Vec<[f64;2]>` --( Dimension shift, flatten! )--> `Vec<f64>`
//!   - Of course, it's same too.
//!
//! ## Implemented features
//!
//! 1. `VecDimensionShift2D`, `VecDimensionShift2DFlatten` and the 2..16-dimension `trait`s for `Vec<T>`.
//! 2. `make_vec_dimension_shift_n_dimension!` macro for make your desired N-dimension `trait`s.
//!
//! Note: In the default, 2D, 3D, 4D version of `VecDimensionShift?D` is enabled. Set `default-features=false` if you don't need these.
//!
//! ## Example
//!
//! ```toml
//! [dependencies]
//! vec-dimension-shift = "*"
//! ```
//!
//! ### Example-1
//!
//! 1D -> 2D -> 1D -> 3D -> 1D, and modify an element:
//!
//! ```rust
//! use vec_dimension_shift::{
//!  VecDimensionShift2D,
//!  VecDimensionShift2DFlatten,
//!  VecDimensionShift3D,
//!  VecDimensionShift3DFlatten
//! };
//!
//! fn d2_and_d3()
//! {
//!  let original = vec![0.0, 1.1, 2.2, 3.3, 4.4, 5.5];
//!  dbg!(&original);
//!
//!  let mut d2_shifted = original.as_2d_array().unwrap();
//!  dbg!(&d2_shifted);
//!  assert_eq!(d2_shifted[0], [0.0, 1.1]);
//!  assert_eq!(d2_shifted[1], [2.2, 3.3]);
//!  assert_eq!(d2_shifted[2], [4.4, 5.5]);
//!  d2_shifted[1][1] = -1.0;
//!
//!  let flatten = d2_shifted.as_flatten();
//!  dbg!(&flatten);
//!
//!  let mut d3_shifted = flatten.as_3d_array().unwrap();
//!  dbg!(&d3_shifted);
//!  assert_eq!(d3_shifted[0], [0.0, 1.1, 2.2]);
//!  assert_eq!(d3_shifted[1], [-1.0, 4.4, 5.5]);
//!  d3_shifted[1][1] = -2.0;
//!
//!  let flatten = d3_shifted.as_flatten();
//!  dbg!(&flatten);
//!
//!  assert_eq!(flatten, vec![0.0, 1.1, 2.2, -1.0, -2.0, 5.5])
//! }
//! ```
//!
//! ### Example-2
//!
//! 1. Make `trait`s just you needs (eg, 2D and 3D)
//! 2. -> Make 1D * 12-length buffer
//! 3. -> Shift the dimension to 2D * 6-length buffer
//! 4. -> Shift the dimension to ( 2D * 3D ) * 2-length buffer
//!
//! ```rust
//! use vec_dimension_shift::make_vec_dimension_shift_n_dimension;
//!
//! fn n_dimension_macro_generator()
//! {
//!  make_vec_dimension_shift_n_dimension! { VecDimensionShift2D, VecDimensionShift2DFlatten, as_2d_array_no_check, to_2d_array_no_check, as_2d_array, to_2d_array, as_2d_array_truncate, to_2d_array_truncate, as_2d_array_padding, to_2d_array_padding, 2 }
//!  make_vec_dimension_shift_n_dimension! { VecDimensionShift3D, VecDimensionShift3DFlatten, as_3d_array_no_check, to_3d_array_no_check, as_3d_array, to_3d_array, as_3d_array_truncate, to_3d_array_truncate, as_3d_array_padding, to_3d_array_padding, 3 }
//!
//!  let original = vec![0.0, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8, 9.9, 10.10, 11.11];
//!  dbg!(&original);
//!
//!  let d2 = original.as_2d_array().unwrap();
//!  assert_eq!(d2[0], [0.0, 1.1]);
//!  assert_eq!(d2[1], [2.2, 3.3]);
//!  assert_eq!(d2[2], [4.4, 5.5]);
//!  assert_eq!(d2[3], [6.6, 7.7]);
//!  assert_eq!(d2[4], [8.8, 9.9]);
//!  assert_eq!(d2[5], [10.10, 11.11]);
//!  dbg!(&d2);
//!
//!  let d3 = d2.as_3d_array().unwrap();
//!  assert_eq!(d3[0], [[0.0, 1.1], [2.2, 3.3], [4.4, 5.5]]);
//!  assert_eq!(d3[1], [[6.6, 7.7], [8.8, 9.9], [10.10, 11.11]]);
//!  dbg!(&d3);
//! }
//! ```
//!
//! More details and examples are exists in the [README.md][] and [examples/] and [tests/].
//!
//! [README.md]: https://github.com/usagi/vec-dimension-shift
//! [examples/]: https://github.com/usagi/vec-dimension-shift/blob/master/examples
//! [tests/]: https://github.com/usagi/vec-dimension-shift/blob/master/tests
//!

#[cfg(feature = "d2")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift2D, VecDimensionShift2DFlatten, as_2d_array_no_check, to_2d_array_no_check, as_2d_array, to_2d_array, as_2d_array_truncate, to_2d_array_truncate, as_2d_array_padding, to_2d_array_padding, 2 }
#[cfg(feature = "d3")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift3D, VecDimensionShift3DFlatten, as_3d_array_no_check, to_3d_array_no_check, as_3d_array, to_3d_array, as_3d_array_truncate, to_3d_array_truncate, as_3d_array_padding, to_3d_array_padding, 3 }
#[cfg(feature = "d4")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift4DFlatten, as_4d_array_no_check, to_4d_array_no_check, as_4d_array, to_4d_array, as_4d_array_truncate, to_4d_array_truncate, as_4d_array_padding, to_4d_array_padding, 4 }
#[cfg(feature = "d5")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift5DFlatten, as_5d_array_no_check, to_5d_array_no_check, as_5d_array, to_5d_array, as_5d_array_truncate, to_5d_array_truncate, as_5d_array_padding, to_5d_array_padding, 4 }
#[cfg(feature = "d6")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift6DFlatten, as_6d_array_no_check, to_6d_array_no_check, as_6d_array, to_6d_array, as_6d_array_truncate, to_6d_array_truncate, as_6d_array_padding, to_6d_array_padding, 4 }
#[cfg(feature = "d7")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift7DFlatten, as_7d_array_no_check, to_7d_array_no_check, as_7d_array, to_7d_array, as_7d_array_truncate, to_7d_array_truncate, as_7d_array_padding, to_7d_array_padding, 4 }
#[cfg(feature = "d8")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift8DFlatten, as_8d_array_no_check, to_8d_array_no_check, as_8d_array, to_8d_array, as_8d_array_truncate, to_8d_array_truncate, as_8d_array_padding, to_8d_array_padding, 4 }
#[cfg(feature = "d9")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift9DFlatten, as_9d_array_no_check, to_9d_array_no_check, as_9d_array, to_9d_array, as_9d_array_truncate, to_9d_array_truncate, as_9d_array_padding, to_9d_array_padding, 4 }
#[cfg(feature = "d10")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift10DFlatten, as_10d_array_no_check, to_10d_array_no_check, as_10d_array, to_10d_array, as_10d_array_truncate, to_10d_array_truncate, as_10d_array_padding, to_10d_array_padding, 4 }
#[cfg(feature = "d11")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift11DFlatten, as_11d_array_no_check, to_11d_array_no_check, as_11d_array, to_11d_array, as_11d_array_truncate, to_11d_array_truncate, as_11d_array_padding, to_11d_array_padding, 4 }
#[cfg(feature = "d12")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift12DFlatten, as_12d_array_no_check, to_12d_array_no_check, as_12d_array, to_12d_array, as_12d_array_truncate, to_12d_array_truncate, as_12d_array_padding, to_12d_array_padding, 4 }
#[cfg(feature = "d13")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift13DFlatten, as_13d_array_no_check, to_13d_array_no_check, as_13d_array, to_13d_array, as_13d_array_truncate, to_13d_array_truncate, as_13d_array_padding, to_13d_array_padding, 4 }
#[cfg(feature = "d14")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift14DFlatten, as_14d_array_no_check, to_14d_array_no_check, as_14d_array, to_14d_array, as_14d_array_truncate, to_14d_array_truncate, as_14d_array_padding, to_14d_array_padding, 4 }
#[cfg(feature = "d15")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift15DFlatten, as_15d_array_no_check, to_15d_array_no_check, as_15d_array, to_15d_array, as_15d_array_truncate, to_15d_array_truncate, as_15d_array_padding, to_15d_array_padding, 4 }
#[cfg(feature = "d16")]
make_vec_dimension_shift_n_dimension! { VecDimensionShift4D, VecDimensionShift16DFlatten, as_16d_array_no_check, to_16d_array_no_check, as_16d_array, to_16d_array, as_16d_array_truncate, to_16d_array_truncate, as_16d_array_padding, to_16d_array_padding, 4 }

/// This is the `trait` generator. If you need the N-dimension version, then use it.
/// (Note: 2D, 3D, 4D are available on the features. )
/// ## Usage
/// Code:
/// ```rust,ignore
/// make_vec_dimension_shift_n_dimension! { VecDimensionShift2D, VecDimensionShift2DFlatten, as_2d_array_no_check, to_2d_array_no_check, as_2d_array, to_2d_array, as_2d_array_truncate, to_2d_array_truncate, as_2d_array_padding, to_2d_array_padding, 2 }
/// ```
/// Then you will get the `VecDimensionShift2D` trait.
/// Thus, you can use the dimension shift implements such as:
/// ```rust,ignore
/// let my_vec = vec![0,1,2,3,4,5];
/// println!("my_vec is: {}", my_vec);
/// let my_2d_shifted = my_vec.as_2d_array().unwrap();
/// println!("my_2d_view is: {}", my_2d_view);
/// let my_flatten = my_2d_shifted.as_flatten();
/// println!("my_flatten is: {}", my_flatten);
/// let my_3d_view = my_flatten.as_3d_array().unwrap();
/// println!("my_3d_view is: {}", my_3d_view);
/// ```
/// It can be simplify if the `#![feature(concat_idents)]` to stable, maybe.
#[macro_export]
macro_rules! make_vec_dimension_shift_n_dimension {
 (
  $trait_symbol:ident,
  $trait_flatten_symbol:ident,
  $as_nc:ident,
  $to_nc:ident,
  $as_c:ident,
  $to_c:ident,
  $as_truncate:ident,
  $to_truncate:ident,
  $as_padding:ident,
  $to_padding:ident,
  $dimension:expr
 ) => {
  pub trait $trait_flatten_symbol<T>
  {
   fn as_flatten(self) -> Vec<T>;
  }
  impl<T> $trait_flatten_symbol<T> for Vec<[T; $dimension]>
  {
   fn as_flatten(self) -> Vec<T>
   {
    const ADDRESS_OFFSET_OF_VEC_SIZE: usize = 2;
    let flatten_len = $dimension * self.len();
    let r: Vec<T> = unsafe { std::mem::transmute(self) };
    let r_ptr: *const usize = unsafe { std::mem::transmute(&r) };
    let r_len_ptr: usize = r_ptr as usize + ADDRESS_OFFSET_OF_VEC_SIZE * std::mem::size_of::<usize>();
    let r_len_ptr: *mut usize = unsafe { std::mem::transmute(r_len_ptr) };
    unsafe {
     std::ptr::write(r_len_ptr, flatten_len);
    }
    r
   }
  }
  pub trait $trait_symbol<T>
  {
   fn $as_nc(self) -> Vec<[T; $dimension]>;
   fn $to_nc(&self) -> Vec<[T; $dimension]>;
   fn $as_c(self) -> Result<Vec<[T; $dimension]>, Vec<T>>;
   fn $to_c(&self) -> Result<Vec<[T; $dimension]>, &Vec<T>>;
   fn $as_truncate(self) -> Vec<[T; $dimension]>;
   fn $to_truncate(&mut self) -> Vec<[T; $dimension]>;
   fn $as_padding(self, v: T) -> Vec<[T; $dimension]>;
   fn $to_padding(&mut self, v: T) -> Vec<[T; $dimension]>;
  }
  impl<T: Clone + Default> $trait_symbol<T> for Vec<T>
  {
   fn $as_nc(self) -> Vec<[T; $dimension]>
   {
    let mut r: Vec<[T; $dimension]> = unsafe { std::mem::transmute(self) };
    r.truncate(r.len() / $dimension);
    r
   }

   fn $to_nc(&self) -> Vec<[T; $dimension]>
   {
    let mut r: Vec<[T; $dimension]> = unsafe { std::ptr::read(self.as_ptr() as _) };
    r.truncate(r.len() / $dimension);
    r
   }

   fn $as_c(self) -> Result<Vec<[T; $dimension]>, Vec<T>>
   {
    const D: usize = $dimension;
    match self.len() % D == 0
    {
     true => Ok(self.$as_nc()),
     _ => Err(self)
    }
   }

   fn $to_c(&self) -> Result<Vec<[T; $dimension]>, &Vec<T>>
   {
    const D: usize = $dimension;
    match self.len() % D == 0
    {
     true => Ok(self.$to_nc()),
     _ => Err(self)
    }
   }

   fn $as_truncate(mut self) -> Vec<[T; $dimension]>
   {
    const D: usize = $dimension;
    self.truncate(self.len() - self.len() % D);
    self.$as_nc()
   }

   fn $to_truncate(&mut self) -> Vec<[T; $dimension]>
   {
    const D: usize = $dimension;
    self.truncate(self.len() - self.len() % D);
    self.$to_nc()
   }

   fn $as_padding(mut self, v: T) -> Vec<[T; $dimension]>
   {
    const D: usize = $dimension;
    self.resize(self.len() + D - self.len() % D, v);
    self.$as_nc()
   }

   fn $to_padding(&mut self, v: T) -> Vec<[T; $dimension]>
   {
    const D: usize = $dimension;
    self.resize(self.len() + D - self.len() % D, v);
    self.$to_nc()
   }
  }
 };
}
