// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use std::cmp;
use libc::c_int;
use num::complex::{
    Complex32,
    Complex64,
};
use ll::*;
use matrix::{
    Matrix,
};
use scalar::Scalar;

pub trait Gels {
    fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>);
}

macro_rules! least_sq_impl(($($t: ident), +) => ($(
    impl Gels for $t {
        fn gels(a: &mut Matrix<Self>, b: &mut Matrix<Self>) {
            unsafe {
                let mut info: c_int = 0;

                let m = a.rows();
                let n = a.cols();
                let nrhs = b.cols();
                let mn = cmp::min(m, n);
                let work_len = mn + cmp::max(mn, nrhs);
                let mut work: Vec<$t> = Vec::with_capacity(work_len as usize);
                work.set_len(work_len as usize);

                prefix!($t, gels_)(a.transpose().as_i8().as_mut(),
                    m.as_mut(), n.as_mut(),
                    nrhs.as_mut(),
                    a.as_mut_ptr(), a.rows().as_mut(),
                    b.as_mut_ptr(), b.rows().as_mut(),
                    (&mut work[..]).as_mut_ptr(), work_len.as_mut(),
                    &mut info as *mut c_int);
            }
        }
    }
)+));

least_sq_impl!(f32, f64, Complex32, Complex64);
