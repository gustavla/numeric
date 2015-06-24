macro_rules! add_impl {
    ($t:ty, $m:ident) => (
        mod $m {
            #[allow(unused_imports)]
            use tensor::Tensor;
            type T = Tensor<$t>;

            #[test]
            fn tensor_tensor_1() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0, 1.0, 8.0,  0.0, -1.0, 2.0]).reshaped(&[2, 3]);
                let t = t1 - t2;
                assert!(t == T::new(vec![2.0, 2.0, -6.0, 10.0, -2.0, -2.0]).reshaped(&[2, 3]));
            }

            #[test]
            fn tensor_tensor_2() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0, 1.0, 8.0,  0.0, -1.0, 2.0]).reshaped(&[2, 3]);
                let t = t1 - &t2;
                assert!(t == T::new(vec![2.0, 2.0, -6.0, 10.0, -2.0, -2.0]).reshaped(&[2, 3]));
            }

            #[test]
            fn tensor_tensor_3() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0, 1.0, 8.0,  0.0, -1.0, 2.0]).reshaped(&[2, 3]);
                let t = &t1 - &t2;
                assert!(t == T::new(vec![2.0, 2.0, -6.0, 10.0, -2.0, -2.0]).reshaped(&[2, 3]));
            }

            #[test]
            #[should_panic]
            fn tensor_tensor_mismatched_1() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0, 1.0, 8.0,  0.0]).reshaped(&[2, 2]);
                t1 - t2; // mis-matched shape
            }

            /*
            #[test]
            fn tensor_tscalar_1() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0]).reshaped(&[2, 3]);
                let t = t1 - t2;
                assert!(t == T::new(vec![-2.0, 1.0, 0.0, 8.0, -5.0, -2.0]).reshaped(&[2, 3]));
            }

            #[test]
            fn tensor_tscalar_1() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t2 = T::new(vec![-2.0]).reshaped(&[2, 3]);
                let t = t1 - t2;
                assert!(t == T::new(vec![-2.0, 1.0, 0.0, 8.0, -5.0, -2.0]).reshaped(&[2, 3]));
            }
            */


            #[test]
            fn tensor_scalar_1() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t = t1 - 3.0;
                assert!(t == T::new(vec![-3.0, 0.0, -1.0, 7.0, -6.0, -3.0]).reshaped(&[2, 3]));
            }

            /*
            #[test]
            fn tensor_scalar_2() {
                let t1 = T::new(vec![ 0.0, 3.0, 2.0, 10.0, -3.0, 0.0]).reshaped(&[2, 3]);
                let t = &t1 - 3.0;
                println!("{:?}", t1.shape());
                assert!(t == T::new(vec![3.0, 6.0, 5.0, 13.0, 0.0, 3.0]).reshaped(&[2, 3]));
            }
            */
        }
    )
}

add_impl!(f32, float32);
add_impl!(f64, float64);