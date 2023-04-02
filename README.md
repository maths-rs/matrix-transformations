
# matrix-transformations.rs
[![License](https://img.shields.io/badge/license-MIT%20%26%20Apache%202.0-green)](#license)
[![CI](https://github.com/graphics-rs/matrix-transformations/actions/workflows/main.yml/badge.svg)](https://github.com/graphics-rs/matrix-transformations/actions/workflows/main.yml)
[![Security audit](https://github.com/graphics-rs/matrix-transformations/actions/workflows/security-audit.yml/badge.svg)](https://github.com/graphics-rs/matrix-transformations/actions/workflows/security-audit.yml)
> warning: **matrix-transformations.rs is currently experimental**
This crate will provide vector and matrix operations.

## Install
```shell
cargo add matrix-transformations
```
or, simply add the following string to your Cargo.toml:

```toml
matrix-transformations = "0.0.0"
```

## Features
* `Fsize` type: Similar to usize and isize but for floats.
* `VectorSD` type: A single dimensional vector. 
* `VectorMD` type: A multi dimensional vector. 
* `Matrix2D` type: A two dimensional matrix in homogeneous coordinates.
* `Matrix3D` type: A three dimensional matrix in homogeneous coordinates.
* `Point2D` type: A two dimensional point in homogeneous coordinates.
* `Point3D` type: A three dimensional point in homogeneous coordinates.
* `I4` const: A identity matrix size 4x4
* `I3` const: A identity matrix size 3x3
* `VecScalingProjection` trait: Implements  `magnitude()`, `vec_scalar_components()`, and `vec_projection()` for vectors. `magnitude()` returns a scalar while `vec_scalar_components()` and `vec_projection()` return a new vector.
* `VectorOps` trait: Implements `vec_scal()`, `vec_add()` and `dot()` for vectors. `dot()` returns a scalar while `vec_scal()`, `vec_add()` return a new vector.

## How it works
* `vec_scal()` Multiplies a vector by a scalar. Defined as $$c\vec{v} = [ cv_{0} ,cv_{1},cv_{2}... cv_{n-1} ]^{T} \in \mathbb{R}^{n} \quad\forall\vec{v}\in\mathbb{R}^{n},c\in \mathbb{R}$$  
* `vec_add` Adds two vectors together. Define as $$\vec{u}+\vec{v} = [ u_{0}+v_{0}, u_{1}+v_{1}, u_{2}+v_{2}...a_{n-1}+b_{n-1}]^{T} \in\mathbb{R}^{n} \quad\forall\vec{u},\vec{v}\in\mathbb{R}^{n}$$
*  `dot()` Performs the dot operation of two vectors. Define as $$\vec{u}\cdot\vec{v}= d=\sum_{i=0}^{n-1}u_{i}v_{i} \quad\forall\vec{u}\vec{v}\in\mathbb{R}^{n}, d\in\mathbb{R}$$
* `magnitude()` gets the magnitude of a vector. Defined by $$\sqrt{\sum_{i=0}^{n-1}\vec{v}_{i}^{2} }$$
* `vec_scalar_components()` Finds the scaler component of  $\vec{u}$ along  $\vec{v}$. It is computed with the following formula $$\frac{\vec{u} \cdot \vec{v}}{	\vert \vec{v} \vert}$$
* `vec_projection()` Finds the vector projection of $\vec{a}$ onto  $\vec{b}$. It is computed with the following formula $$(\frac{\vec{u} \cdot \vec{v}}{	\vert \vec{v} \vert})\vec{v}$$

## Examples

Performing scaling on a vector
```rs
use matrix_transformations::{Fsize, VectorOps};

fn  main() {
	let vec_1:  Vec<Fsize> =  vec![1.0, 1.0, 4.0];
	let vec_2:  Vec<Fsize> =  vec![4.0, 1.1, 4.5, 1.4];
	
	assert_eq!(vec![3.0, 3.0, 12.0], vec_1.vec_scal(3.0));
	assert_eq!(vec![-4.0, -1.1, -4.5, -1.4], vec_2.vec_scal(-1.0));
}
```

Performing vector addition
```rs
use matrix_transformations::{Fsize, VectorOps};

fn  main() {
	let vec_1:  Vec<Fsize> =  vec![1.0, 1.0, 4.0];
	let vec_2:  Vec<Fsize> =  vec![2.0, 3.0, 4.1];

	let vec_3:  Vec<Fsize> =  vec![3.9, 3.9, 4.0, 3.1];
	let vec_4:  Vec<Fsize> =  vec![4.0, 1.1, 4.5, 1.4];

	assert_eq!(vec![3.0, 4.0, 8.1], vec_1.vec_add(&vec_2));
	assert_eq!(vec![7.9, 5.0, 8.5, 4.5], vec_3.vec_add(&vec_4));
}
```

Performing the dot operation
```rs
use matrix_transformations::{Fsize, VectorOps};

fn main() {
	let vec_1: Vec<Fsize> = vec![1.0, 2.0, 3.0];
	let vec_2: Vec<Fsize> = vec![3.0, 4.0, 4.0];

	let vec_7: Vec<Fsize> = vec![1.0, 2.0, 1.0, 1.0, 5.0];
	let vec_8: Vec<Fsize> = vec![3.0, 4.0, 4.9, 1.4, 4.1];

	assert_eq!(23.0, vec_1.dot(&vec_2));
	assert_eq!(37.8, vec_7.dot(&vec_8));
}
```

Getting the magnitude of a vector 
```rs
use matrix_transformations::{Fsize, VecScalingProjection};

fn main() {
    let vec_1: Vec<Fsize> = vec![1.0, 2.0, 3.0, 4.0];
    let vec_2: Vec<Fsize>  = vec![3.0, 1.3, 4.1];
    let vec_3: Vec<Fsize>  = vec![1.4, 1.4, 14.5, 14.4, 2.0];

    assert_eq!((30.0 as Fsize).sqrt(), vec_1.magnitude());
    assert_eq!((27.5 as Fsize).sqrt(), vec_2.magnitude());
    assert_eq!((425.53 as Fsize).sqrt(), vec_3.magnitude());
}
```


Finding the scalar component
```rs
use matrix_transformations::{Fsize, VecScalingProjection};

fn main() {
	let vec_1: Vec<Fsize> = vec![1.0, 3.0];
	let vec_2: Vec<Fsize> = vec![2.0, 1.0];

	let vec_3: Vec<Fsize> = vec![3.0, 4.0];
	let vec_4: Vec<Fsize> = vec![5.0, -12.0];

	assert_eq!((5.0 as Fsize).sqrt(), vec_1.vec_scalar_components(&vec_2));
	assert_eq!((-33.0 / 13.0 as Fsize), vec_3.vec_scalar_components(&vec_4));
}
```
> **Note**:  We are getting the scalar component of `vec_1` along `vec_2` and `vec_3` along `vec_4`
Finding the scalar component
```rs
use matrix_transformations::{Fsize, VecScalingProjection};

fn main() {
	let vec_1: Vec<Fsize> = vec![1.0, 3.0];
	let vec_2: Vec<Fsize> = vec![2.0, 1.0];

	let vec_3: Vec<Fsize> = vec![1.0, 2.0];
	let vec_4: Vec<Fsize> = vec![-3.0, 4.0];

	assert_eq!(
		vec![1.9999999999999996, 0.9999999999999998],
		vec_1.vec_projection(&vec_2)
	);
	assert_eq!(
		vec![-0.6000000000000001, 4.0 / 5.0],
		vec_3.vec_projection(&vec_4)
	);
}
```
> **Note**:  We are getting the vector of `vec_1` onto `vec_2` and `vec_3` onto `vec_4`. The funky math results can be explained by computers  limitations of floating-point arithmetic
## License
Licensed under either of
 * Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
