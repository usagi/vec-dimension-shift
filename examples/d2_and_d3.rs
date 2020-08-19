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

fn main()
{
 d2_and_d3();
}
