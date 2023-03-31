#[doc = include_str!("../README.md")]
use num::{Float, Integer};
mod matrix_operations;
mod vector_ops;

use std::ops::Index;

#[cfg(target_pointer_width = "64")]
type Fsize = f64;
#[cfg(target_pointer_width = "32")]
type Fsize = f32;

type VectorSD = Vec<Fsize>;
type VectorMD = Vec<Vec<Fsize>>;

type Matrix3D = [[Fsize; 4]; 4];
type Matrix2D = [[Fsize; 3]; 3];

type Point3D = [Fsize; 4];
type Point2D = [Fsize; 3];

const I4: Matrix3D = [
	[1.0, 0.0, 0.0, 0.0],
	[0.0, 1.0, 0.0, 0.0],
	[0.0, 0.0, 1.0, 0.0],
	[0.0, 0.0, 0.0, 1.0],
];

const I3: Matrix2D = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

pub trait SelfVectorOps<Rhs> {
	type Output;
	fn magnitude(&self) -> Self::Output;
	fn scalar_components(&self, rhs: &Rhs) -> Self::Output;
	fn projection(&self, rhs: &Rhs);
}

pub trait VectorOps<Rhs, A> {
	type Output;
	fn scal_mult(&self, scal: A) -> Vec<Self::Output>;
	fn vec_add(&self, rhs: &Rhs) -> Vec<Self::Output>;
	fn dot(&self, rhs: &Rhs) -> Self::Output;
}
