#[doc = include_str!("../README.md")]
mod matrix_operations;
mod vector_ops;

#[cfg(target_pointer_width = "64")]
pub type Fsize = f64;
#[cfg(target_pointer_width = "32")]
pub type Fsize = f32;

pub type VectorSD = Vec<Fsize>;
pub type VectorMD = Vec<Vec<Fsize>>;

pub type Matrix3D = [[Fsize; 4]; 4];
pub type Matrix2D = [[Fsize; 3]; 3];

pub type Point3D = [Fsize; 4];
pub type Point2D = [Fsize; 3];

pub const I4: Matrix3D = [
	[1.0, 0.0, 0.0, 0.0],
	[0.0, 1.0, 0.0, 0.0],
	[0.0, 0.0, 1.0, 0.0],
	[0.0, 0.0, 0.0, 1.0],
];

pub const I3: Matrix2D = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

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
