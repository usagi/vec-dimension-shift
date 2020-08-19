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

fn main()
{
 n_dimension_macro_generator();
}
